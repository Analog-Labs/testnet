#!/usr/bin/env pwsh
Param(
    [switch]$Up,
    [switch]$Down,
    [switch]$Build,
    [switch]$Upload,
    [switch]$Cargo,
    [switch]$Restart,
    [switch]$Stop,    
    [switch]$Start,
    [switch]$Status
)

Push-Location $PSScriptRoot

if ($Cargo -or -not(Test-path ../cargo.toml.tar -PathType leaf))
{
    ./make.cargo.tar.sh
}

if ($Build -or $Upload)
{
    docker build .. -f ./docker/Dockerfile -t ghcr.io/analog-labs/testnet
}

if ($Upload)
{
    docker push ghcr.io/analog-labs/testnet
}

if ($Down -or $Up)
{
    docker compose down -v
}

if ($Up)
{
    docker compose up -V --force-recreate -d
}

if ($Restart)
{
    Push-Location aws
    if (-not(Test-path ./.terraform))
    {
        terraform init
    }
    terraform apply -destroy -target aws_instance.validator_node -target aws_instance.boot_node -auto-approve
    terraform apply -auto-approve
    Pop-Location
}

if ($Stop)
{
    Push-Location aws
    if (-not(Test-path ./.terraform))
    {
        terraform init
    }
    terraform apply -destroy -target aws_instance.validator_node -target aws_instance.boot_node -auto-approve
    Pop-Location
}

if ($Start)
{
    Push-Location aws
    if (-not(Test-path ./.terraform))
    {
        terraform init
    }
    terraform apply -auto-approve
    Pop-Location
}

if ($Status)
{
    Push-Location aws
    if (-not(Test-path ./.terraform))
    {
        terraform init
    }
    terraform show | sed '0,/^Outputs:$/d'
    Pop-Location
}

Pop-Location
