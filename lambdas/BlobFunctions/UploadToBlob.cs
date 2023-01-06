using Azure.Storage.Blobs;
using System.Threading.Tasks;
using Microsoft.AspNetCore.Mvc;
using Microsoft.Azure.WebJobs;
using Microsoft.Azure.WebJobs.Extensions.Http;
using Microsoft.AspNetCore.Http;
using Microsoft.Extensions.Logging;
using System;
using Azure.Storage.Blobs.Models;

namespace BlobFunctions
{
    public static class UploadToBlob
    {
        [FunctionName("UploadToBlob")]
        public static async Task<IActionResult> Run(
            [HttpTrigger(AuthorizationLevel.Function, "post", Route = null)] HttpRequest req,
            ILogger log)
        {
            var connectionString = "DefaultEndpointsProtocol=https;AccountName=diplomablob;AccountKey=VsdWZBUdqPsS2Lf1PBVheLA3JWOWdrD4iDLA4Hh2uyUzNtuejSOKlAsG5hIzWxh8bZ8zeA49YTQa+ASt1/q4yw==;EndpointSuffix=core.windows.net";
            var file = req.Form.Files["File"];
            var containerName = file.ContentType.Replace("/", "-");
            log.LogInformation(file.Name);
            var fileBlob = file.OpenReadStream();
            log.LogInformation($"{fileBlob.Length}");
            var blobClient = new BlobContainerClient(connectionString, containerName);
            // set access level to public (or understand how does the security key authentication works)
            var blobCreation = await blobClient.CreateIfNotExistsAsync();
            if (blobCreation?.GetRawResponse() is not null && blobCreation.GetRawResponse().IsError)
            {
                return new StatusCodeResult(500);
            }
            log.LogInformation("lol1");
            var blob = blobClient.GetBlobClient($"{Guid.NewGuid().ToString()}.mp4");
            log.LogInformation("lol2");
            await blob.UploadAsync(fileBlob, new BlobUploadOptions {HttpHeaders = new BlobHttpHeaders {ContentType = file.ContentType}});
            log.LogInformation("lol3");
            return new OkObjectResult("file uploaded successfully");
        }
    }
}
