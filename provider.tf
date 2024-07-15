terraform {
  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "~> 5.47.0"
    }

    random = {
      source  = "hashicorp/random"
      version = "~> 3.6.1"
    }

    tls = {
      source  = "hashicorp/tls"
      version = "~> 4.0.5"
    }

    cloudinit = {
      source  = "hashicorp/cloudinit"
      version = "~> 2.3.4"
    }
  }

  required_version = "~> 1.3"
}


# Data for AWS Availability Zones
data "aws_availability_zones" "available" {
  filter {
    name   = "opt-in-status"
    values = ["opt-in-not-required"]
  }
}


provider "aws" {
  region  = "us-east-1"
}

terraform {
  backend "s3" {
    bucket = "aws-terraform-state-storage"
    key    = "lambda-pagamentos/terraform.tfstate"
    region = "us-east-1"
  }
}