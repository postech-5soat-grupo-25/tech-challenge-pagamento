import json
import boto3
from datetime import datetime
from aws_lambda_powertools import Logger
from botocore.exceptions import ClientError


# Configuração do logger
logger = Logger(service="LambdaPagamentos")

dynamodb = boto3.resource('dynamodb')
table = dynamodb.Table('tabela_pagamentos')

# Constantes
ENDPOINT_NOT_FOUND = {
    'statusCode': 404,
    'body': json.dumps({'message': 'Endpoint não encontrado'})
}
SUCCESS_MESSAGE = 'message'
ERROR_MESSAGE = 'error'

def lambda_handler(event, context):
    path = event.get('path')
    http_method = event.get('httpMethod')
    
    if path == "/pedidos/pagamento" and http_method == "POST":
        logger.info("Inicio da criação de pagamento")
        return criar_pagamento(event)
    elif path.startswith("/pedidos/webhook/") and path.endswith("/pagamento") and http_method == "POST":
        logger.info("Inicio da atualização de status de pagamento")
        return atualizar_status_pagamento(event)
    else:
        return ENDPOINT_NOT_FOUND

def criar_pagamento(event):
    try:
        body = json.loads(event['body'])
    except Exception as e:
        return {
            'statusCode': 400,
            'body': json.dumps({ERROR_MESSAGE: 'Erro no corpo da requisição', 'error': str(e)})
        }

    try:
        required_fields = ['id', 'id_pedido', 'estado', 'valor', 'metodo']
        
        # Validação de entrada
        for field in required_fields:
            if field not in body:
                return {
                    'statusCode': 400,
                    'body': json.dumps({ERROR_MESSAGE: f'Campo {field} é obrigatório'})
                }
        
        item = {
            'id': body['id'],
            'id_pedido': body['id_pedido'],
            'estado': body['estado'],
            'valor': body['valor'],
            'metodo': body['metodo'],
            'referencia': body.get('referencia', None),
            'data_criacao': datetime.now().isoformat()
        }
        
        table.put_item(Item=item)
        return {
            'statusCode': 200,
            'body': json.dumps({SUCCESS_MESSAGE: 'Pagamento criado com sucesso'})
        }
    except ClientError as e:
        logger.error(f"Erro ao criar pagamento: {e}")
        return {
            'statusCode': 500,
            'body': json.dumps({ERROR_MESSAGE: str(e)})
        }
    except json.JSONDecodeError:
        return {
            'statusCode': 400,
            'body': json.dumps({ERROR_MESSAGE: 'Formato de JSON inválido'})
        }

def atualizar_status_pagamento(event):
    try:
        path_parameters = event.get('pathParameters', {})
        id = path_parameters.get('id')
        body = json.loads(event['body'])
        
        if not id:
            return {
                'statusCode': 400,
                'body': json.dumps({ERROR_MESSAGE: 'ID é obrigatório'})
            }
        
        if 'estado' not in body:
            return {
                'statusCode': 400,
                'body': json.dumps({ERROR_MESSAGE: 'Campo estado é obrigatório'})
            }
        
        response = table.update_item(
            Key={'id': int(id)},
            UpdateExpression="set estado = :estado",
            ExpressionAttributeValues={':estado': body['estado']},
            ReturnValues="UPDATED_NEW"
        )
        return {
            'statusCode': 200,
            'body': json.dumps({SUCCESS_MESSAGE: 'Status atualizado com sucesso'})
        }
    except ClientError as e:
        logger.error(f"Erro ao atualizar status do pagamento: {e}")
        return {
            'statusCode': 500,
            'body': json.dumps({ERROR_MESSAGE: str(e)})
        }
    except json.JSONDecodeError:
        return {
            'statusCode': 400,
            'body': json.dumps({ERROR_MESSAGE: 'Formato de JSON inválido'})
        }