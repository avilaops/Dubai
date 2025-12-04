# MongoDB Atlas Sync via HTTP
$apiKey = $env:MONGODB_ATLAS_API_KEY
$appId = "application-0-djjmn"
$baseUrl = "https://data.mongodb-api.com/app/$appId/endpoint/data/v1/action"

$headers = @{
    "Content-Type" = "application/json"
    "api-key" = $apiKey
}

Write-Host "Loading JSON..."
$jsonData = Get-Content "docs/data/dubai-properties.json" -Raw | ConvertFrom-Json

Write-Host "Syncing properties..."
foreach ($property in $jsonData.properties) {
    $body = @{
        dataSource = "cluster0"
        database = "dubai"
        collection = "properties"
        document = $property
    } | ConvertTo-Json -Depth 10 -Compress
    
    try {
        Invoke-RestMethod -Uri "$baseUrl/insertOne" -Method Post -Headers $headers -Body $body | Out-Null
        Write-Host "." -NoNewline
    } catch {
        Write-Host "x" -NoNewline
    }
}

Write-Host "`nDone!"
