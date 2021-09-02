## Forte Test
Create an API client that sends a POST request with auth containing a json payload in the following format
{ "position" : {
"latitude": 45,
"longitude": 120
},
"status": "someStatus"
}
status can assume the following values: AtDestination, BeingPrepared, Delayed and InTransit
for the sake of simplicity latitude and longitude can assume just integer values
for HTTP request the candidate can use any library
some constraints: API server is expecting the status in snake_case and the candidate must include a test to prove that the payload is being serialized correctly
must include a test to prove that the created library is sending the correct headers
bonus: include a retry mechanism on the library to retry failing POST requests