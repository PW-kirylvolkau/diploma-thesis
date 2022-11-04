#r "nuget: Docker.DotNet"

// library works using https://docs.docker.com/engine/api/v1.41/ 
open Docker.DotNet
open Docker.DotNet.Models

let ilist (x: _ array) = x :> System.Collections.Generic.IList<_>
let (!!) = ilist

let config = new DockerClientConfiguration()
let client = config.CreateClient()

let postgresPort = PortBinding()
postgresPort.HostPort <- "5432"
let hostConfig = HostConfig()
hostConfig.PortBindings <- Map.empty.Add("5432", !![|postgresPort|])


let containerConfig = CreateContainerParameters()
containerConfig.Image <- "postgres"
containerConfig.Name <- "local-postgres-2"
containerConfig.Env <- !![|"POSTGRES_PASSWORD=my_password"|]
containerConfig.AttachStderr <- true
containerConfig.AttachStdout <- true
containerConfig.HostConfig <- hostConfig  

let createResponse = 
    containerConfig
        |> client.Containers.CreateContainerAsync
        |> Async.AwaitTask
        |> Async.RunSynchronously

printf "%A" createResponse

let startResponse = 
    client.Containers.StartContainerAsync("local-postgres-2", ContainerStartParameters())
    |> Async.AwaitTask
    |> Async.RunSynchronously

printf "%A" startResponse