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
            // var connectionString = Environment.GetEnvironmentVariable("BlobConnectionString");
            var connectionString = "DefaultEndpointsProtocol=https;AccountName=diplomablob;AccountKey=VsdWZBUdqPsS2Lf1PBVheLA3JWOWdrD4iDLA4Hh2uyUzNtuejSOKlAsG5hIzWxh8bZ8zeA49YTQa+ASt1/q4yw==;EndpointSuffix=core.windows.net";
            var file = req.Form.Files["File"];

            if (file is null)
            {
                return new BadRequestObjectResult(new { msg = $"No file attached." });
            }

            var containerName = file.ContentType switch
            {
                "video/mp4" => "lesson-videos",
                "application/pdf" => "lesson-attachments",
                var str when str.Contains("image") => "pictures",
                _ => null
            };

            if (containerName is null)
            {
                return new BadRequestObjectResult(new { msg = $"{file.ContentType} is not recognised type." });
            }

            var fileBlob = file.OpenReadStream();

            var blobClient = new BlobContainerClient(connectionString, containerName);

            var blobName = $"{Guid.NewGuid()}.{file.FileName.Substring(file.FileName.LastIndexOf(".") + 1)}";
            var blob = blobClient.GetBlobClient(blobName);

            var uploadResult = await blob.UploadAsync(
                fileBlob,
                new BlobUploadOptions
                {
                    HttpHeaders = new BlobHttpHeaders
                    {
                        ContentType = file.ContentType
                    }
                });

            return new OkObjectResult(blobName);
        }
    }
}
