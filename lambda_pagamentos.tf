resource "aws_lambda_function" "lambda_pagamentos" {
  filename         = "dummy_lambda_package.zip"
  function_name    = "LambdaPagamentos"
  role             = aws_iam_role.lambda_pagamentos_role.arn
  handler          = "main.lambda_handler"
  runtime          = "python3.11"
  source_code_hash = filebase64sha256("dummy_lambda_package.zip")

  environment {
    variables = {
      STAGE = "prod"
      PEDIDOS_PRODUTOS_LB_URL = "http://ab8eb731e740f454785ce158a4984454-1283436871.us-east-1.elb.amazonaws.com:4000"
      MOCK_PAGAMENTOS_LB_URL = "12545"
    }
  }
}
