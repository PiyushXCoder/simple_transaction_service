# API Documentation

## Create Account

**Request**

```bash
curl --request POST \
  --url http://localhost:8000/create_account \
  --header 'Authorization: Bearer <API KEY>' \
  --header 'Content-Type: application/json' \
  --data '{
	"username": "<username>",
	"name": "<full name>"
}'
```

**Response**

```json
{
 "message": "Account created successfully"
}
```



## Get Account

**Request**

```bash
curl --request GET \
  --url 'http://localhost:8000/get_account?username=raju' \
  --header 'Authorization: Bearer <API KEY>' 
```

**Response**

```json
{
	"username": "<username>",
	"name": "<Full Name>",
	"balance": <balance>
}
```

## Add Webhook

**Request**

```bash
curl --request POST \
  --url http://localhost:8000/add_webhook \
  --header 'Authorization: Bearer <API KEY>' \
  --header 'Content-Type: application/json' \
  --data '{
	"listening_account": "<username>",
	"url": "http://localhost:3000/"
}'
```

**Response**

```json
{
	"message": "Added webhook"
}
```

## Credit Account

**Request**

```bash
curl --request POST \
  --url http://localhost:8000/credit_account \
  --header 'Authorization: Bearer <API KEY>' \
  --header 'Content-Type: application/json' \
  --data '{
	"receiver": "<username>",
	"amount": <balance>
}'
```

**Response**

```json
{
	"id": <transaction id>
}
```

## Debit Account

**Request**

```bash
curl --request POST \
  --url http://localhost:8000/debit_account \
  --header 'Authorization: Bearer <API KEY>' \
  --header 'Content-Type: application/json' \
  --data '{
	"receiver": "<username>",
	"amount": <balance>
}'
```

**Response**

```json
{
	"id": <transaction id>
}
```



## Transfer

**Request**

```bash
curl --request POST \
  --url http://localhost:8000/transfer_funds \
  --header 'Authorization: Bearer 7f853b13-fc18-4cbb-a80e-3c6002ef7bb4' \
  --header 'Content-Type: application/json' \
  --data '{
	"sender": "<username>", 
	"receiver": "<username>",
	"amount": <amount>
}'
```

**Response**

```bash
{
	"id": <transaction id>
}
```
