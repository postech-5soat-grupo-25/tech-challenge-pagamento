#!/bin/bash

# Configuration Variables
LAMBDA_FUNCTION_NAME="LambdaPagamentos"
LAMBDA_DIR="./lambda-pagamentos"
ZIP_FILE="lambda_pagamentos.zip"
TEMP_DIR="lambda_temp"
ALIAS_NAME="prod"
API_NAME="tech-challenge-api"
REGION="us-east-1"
AWS_ACCOUNT_ID="739842188003"
S3_BUCKET_NAME="lambda-proxy-deploy"

# Function to check if a directory exists and remove it
remove_dir_if_exists() {
    local dir=$1
    if [ -d $dir ]; then
        echo "Removing existing directory: $dir..."
        rm -rf $dir
    fi
}

# Function to check if a file exists and remove it
remove_file_if_exists() {
    local file=$1
    if [ -f $file ]; then
        echo "Removing existing file: $file..."
        rm -f $file
    fi
}

# Fetch API Gateway ID by name
echo "Fetching API Gateway ID by name..."
API_ID=$(aws apigateway get-rest-apis --query "items[?name=='$API_NAME'].id" --output text --region $REGION)

if [ -z "$API_ID" ]; then
    echo "API Gateway with name '$API_NAME' not found."
    exit 1
fi

echo "API Gateway ID found: $API_ID"

# Prepare temporary directory for packaging
remove_dir_if_exists $TEMP_DIR
remove_file_if_exists $ZIP_FILE

echo "Creating temporary directory..."
mkdir $TEMP_DIR

echo "Copying files to the temporary directory..."
cp -r $LAMBDA_DIR/* $TEMP_DIR

echo "Installing dependencies in the temporary directory..."
pip install -r $TEMP_DIR/requirements.txt -t $TEMP_DIR

echo "Creating ZIP file with the Python code..."
cd $TEMP_DIR
zip -r ../$ZIP_FILE .
cd ..

echo "Removing temporary directory..."
rm -rf $TEMP_DIR

# Upload ZIP file to S3
echo "Uploading ZIP file to S3..."
aws s3 cp $ZIP_FILE s3://$S3_BUCKET_NAME/

# Update existing Lambda function using the ZIP file from S3
echo "Updating Lambda function code..."
aws lambda update-function-code \
    --function-name $LAMBDA_FUNCTION_NAME \
    --s3-bucket $S3_BUCKET_NAME \
    --s3-key $ZIP_FILE

# Wait for Lambda function update to complete
echo "Waiting for Lambda function update to complete..."
while true; do
    STATUS=$(aws lambda get-function-configuration --function-name $LAMBDA_FUNCTION_NAME --query 'LastUpdateStatus' --output text)
    if [ "$STATUS" == "Successful" ]; then
        echo "Lambda function update completed successfully."
        break
    elif [ "$STATUS" == "Failed" ]; then
        echo "Lambda function update failed."
        exit 1
    fi
    echo "Update status: $STATUS. Waiting for 10 seconds..."
    sleep 10
done

# Publish new version of Lambda function
echo "Publishing new version of Lambda function..."
LAMBDA_VERSION=$(aws lambda publish-version --function-name $LAMBDA_FUNCTION_NAME --query 'Version' --output text)

# Check if alias exists and update or create it
echo "Checking if alias $ALIAS_NAME exists..."
ALIAS_EXISTS=$(aws lambda list-aliases --function-name $LAMBDA_FUNCTION_NAME --query "Aliases[?Name=='$ALIAS_NAME'].Name" --output text)

if [ "$ALIAS_EXISTS" == "$ALIAS_NAME" ]; then
    echo "Updating alias $ALIAS_NAME to point to new version..."
    aws lambda update-alias \
        --function-name $LAMBDA_FUNCTION_NAME \
        --name $ALIAS_NAME \
        --function-version $LAMBDA_VERSION
else
    echo "Creating alias $ALIAS_NAME..."
    aws lambda create-alias \
        --function-name $LAMBDA_FUNCTION_NAME \
        --name $ALIAS_NAME \
        --function-version $LAMBDA_VERSION \
        --description "Alias for production version"
fi

# Function to add permissions to Lambda function
add_permission_if_not_exists() {
    local statement_id=$1
    local action=$2
    local source_arn=$3
    local permission_exists=$(aws lambda get-policy --function-name "$LAMBDA_FUNCTION_NAME:$ALIAS_NAME" --query 'Policy' --output text | grep $statement_id)

    if [ -z "$permission_exists" ]; then
        aws lambda add-permission \
            --function-name "$LAMBDA_FUNCTION_NAME:$ALIAS_NAME" \
            --statement-id $statement_id \
            --action lambda:$action \
            --principal apigateway.amazonaws.com \
            --source-arn $source_arn
    else
        echo "Permission $statement_id already exists."
    fi
}

# Add permissions for API Gateway to invoke Lambda function
echo "Adding permissions for API Gateway to invoke Lambda function..."
add_permission_if_not_exists "AllowPedidosPagamentoExecutionFromAPIGateway" "InvokeFunction" "arn:aws:execute-api:$REGION:$AWS_ACCOUNT_ID:$API_ID/*/POST/pedidos/pagamento"


echo "Permissions verified and updated successfully!"

# Integrate API Gateway with Lambda function using the alias
LAMBDA_ALIAS_ARN=$(aws lambda get-alias --function-name $LAMBDA_FUNCTION_NAME --name $ALIAS_NAME --query 'AliasArn' --output text)

integrate_api_gateway() {
    local resource_path=$1
    local http_method=$2
    local resource_id=$(aws apigateway get-resources --rest-api-id $API_ID --query "items[?path=='$resource_path'].id" --output text)
    
    aws apigateway put-integration \
        --rest-api-id $API_ID \
        --resource-id $resource_id \
        --http-method $http_method \
        --type AWS_PROXY \
        --integration-http-method POST \
        --uri arn:aws:apigateway:$REGION:lambda:path/2015-03-31/functions/$LAMBDA_ALIAS_ARN/invocations
}

echo "Integrating API Gateway with Lambda function..."
integrate_api_gateway "/pedidos/pagamento" "POST"


aws apigateway create-deployment --rest-api-id $API_ID --stage-name prod

echo "API Gateway integrations configured successfully!"
echo "Lambda function, alias, and API Gateway integrations updated successfully!"
