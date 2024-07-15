resource "aws_dynamodb_table" "tabela_pagamentos" {
    name           = "tabela_pagamentos"
    billing_mode   = "PAY_PER_REQUEST"
    hash_key       = "id"

    attribute {
        name = "id"
        type = "N"
    }
    attribute {
        name = "id_pedido"
        type = "N"
    }


    global_secondary_index {
        name               = "id_pedido-index"
        hash_key           = "id_pedido"
        projection_type    = "ALL"
        read_capacity      = 5
        write_capacity     = 5
    }

}