#r "nuget: Docker.DotNet"

// library works using https://docs.docker.com/engine/api/v1.41/ 
open System
open System.Linq
open System.IO
open Docker.DotNet
open Docker.DotNet.Models

let ilist (x: _ array) = x :> System.Collections.Generic.IList<_>
let (!!) = ilist

let initSql = File.ReadAllText "./init.sql"

let config = new DockerClientConfiguration()
let client = config.CreateClient()

let postgresPort = PortBinding()
postgresPort.HostPort <- "5432"
let hostConfig = HostConfig()
hostConfig.PortBindings <- Map.empty.Add("5432/tcp", !![|postgresPort|])

let guid = Guid.NewGuid().ToString()
let name = $"postgres-{guid}"

let containerConfig = CreateContainerParameters()
containerConfig.Image <- "postgres"
containerConfig.Name <- name
containerConfig.Env <- !![|"POSTGRES_PASSWORD=my_password"|]
containerConfig.AttachStderr <- true
containerConfig.AttachStdout <- true
containerConfig.HostConfig <- hostConfig

let createResponse = 
    containerConfig
    |> client.Containers.CreateContainerAsync
    |> Async.AwaitTask
    |> Async.RunSynchronously

if createResponse.Warnings.Any() then 
    printf "%A" createResponse

let success = 
    client.Containers.StartContainerAsync(name, ContainerStartParameters())
    |> Async.AwaitTask
    |> Async.RunSynchronously

printfn "%s" (if success then "Container started!" else "Failed to started container")