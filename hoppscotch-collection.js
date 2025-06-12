[
  {
    "v": 8,
    "name": "auth",
    "folders": [],
    "requests": [
      {
        "v": "13",
        "auth": {
          "authType": "inherit",
          "authActive": true
        },
        "body": {
          "body": "{\n  \"email\": \"himanshu.knows.u@gmail.com\",\n  \"password\": \"himanshurapidops\",\n  \"full_name\": \"Chavda Himanshu H\",\n  \"mobile\": \"7624042717\"\n}\n",
          "contentType": "application/json"
        },
        "name": "Register Customer",
        "method": "POST",
        "params": [],
        "headers": [],
        "endpoint": "http://localhost:4000/api/v1/auth/public/register",
        "responses": {},
        "testScript": "",
        "preRequestScript": "",
        "requestVariables": []
      },
      {
        "v": "13",
        "auth": {
          "authType": "inherit",
          "authActive": true
        },
        "body": {
          "body": "{\n\"email\": \"himanshu.knows.u@gmail.com\",\n  \"password\": \"himanshurapidops\"\n}\n",
          "contentType": "application/json"
        },
        "name": "Login Customer",
        "method": "POST",
        "params": [],
        "headers": [],
        "endpoint": "http://localhost:4000/api/v1/auth/public/login",
        "responses": {},
        "testScript": "",
        "preRequestScript": "",
        "requestVariables": []
      }
    ],
    "auth": {
      "authType": "inherit",
      "authActive": true
    },
    "headers": [],
    "_ref_id": "coll_mbtxdvo5_1b219e2e-2cd4-447e-b584-a6846614fcfe"
  },
  {
    "v": 8,
    "name": "Product",
    "folders": [],
    "requests": [
      {
        "v": "13",
        "auth": {
          "token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiI5MDM0YTUxNC0xYmFkLTRlYzAtOTcwNS02ZjEwYWViNTI0ZGEiLCJleHAiOjE3NDk3ODQ2MTJ9.r2Txm60n3k07KWyHYzBCDTDqnRbt2v-c2VMg155t4Cw",
          "authType": "bearer",
          "authActive": true
        },
        "body": {
          "body": "{\n  \"name\": \"lenovo idealpad gameing 3i\",\n  \"brand\": \"lenovo\",\n  \"is_available\": true,\n  \"count_in_stock\": 63,\n  \"category\": \"Laptop\",\n  \"price\": 800.89,\n  \"description\": \"Lenovo idealpad gaming 3i  \",\n  \"images\": [\n    \"www.google4.com\",\n    \"www.google51.com\"\n  ]\n}\n",
          "contentType": "application/json"
        },
        "name": "create products ",
        "method": "POST",
        "params": [],
        "headers": [],
        "endpoint": "http://localhost:4000/api/v1/products/admin/create",
        "responses": {},
        "testScript": "",
        "preRequestScript": "",
        "requestVariables": []
      },
      {
        "v": "13",
        "auth": {
          "token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiI4NjFiYTU3MC04ZThhLTQ2ZDAtYTNjMS00ZGMwZDA1ODdhYzAiLCJleHAiOjE3NDk3MzMzMzV9.tFYbd3bYk5XLY9ebbuvWokeVwE66gKlw7JTZb93Y1kM",
          "authType": "bearer",
          "authActive": true
        },
        "body": {
          "body": "{\n  \"name\": \"Charger evo 2.1\",\n  \"brand\": \"evo\",\n  \"is_available\": true,\n  \"count_in_stock\": 63,\n  \"category\": \"charger\",\n  \"price\": 20.89,\n  \"description\": \"mobile charger \",\n  \"images\": [\n    \"www.google4.com\",\n    \"www.google51.com\"\n  ]\n}\n",
          "contentType": "application/json"
        },
        "name": "update product  ",
        "method": "PUT",
        "params": [],
        "headers": [],
        "endpoint": "http://localhost:4000/api/v1/products/admin/09cfd9af-38e9-4fb4-bda3-0fafc287d79c",
        "responses": {},
        "testScript": "",
        "preRequestScript": "",
        "requestVariables": []
      },
      {
        "v": "13",
        "auth": {
          "token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiI4NjFiYTU3MC04ZThhLTQ2ZDAtYTNjMS00ZGMwZDA1ODdhYzAiLCJleHAiOjE3NDk3MzMzMzV9.tFYbd3bYk5XLY9ebbuvWokeVwE66gKlw7JTZb93Y1kM",
          "authType": "bearer",
          "authActive": true
        },
        "body": {
          "body": null,
          "contentType": null
        },
        "name": "delete product",
        "method": "DELETE",
        "params": [],
        "headers": [],
        "endpoint": "http://localhost:4000/api/v1/products/admin/db798531-023a-4dfc-ad2e-49aceb4d0606",
        "responses": {},
        "testScript": "",
        "preRequestScript": "",
        "requestVariables": []
      },
      {
        "v": "13",
        "auth": {
          "authType": "inherit",
          "authActive": true
        },
        "body": {
          "body": null,
          "contentType": null
        },
        "name": "get all products",
        "method": "GET",
        "params": [],
        "headers": [],
        "endpoint": "http://localhost:4000/api/v1/products/public",
        "responses": {},
        "testScript": "",
        "preRequestScript": "",
        "requestVariables": []
      },
      {
        "v": "13",
        "auth": {
          "authType": "inherit",
          "authActive": true
        },
        "body": {
          "body": null,
          "contentType": null
        },
        "name": "get product by id",
        "method": "GET",
        "params": [],
        "headers": [],
        "endpoint": "http://localhost:4000/api/v1/products/public/09cfd9af-38e9-4fb4-bda3-0fafc287d79c",
        "responses": {},
        "testScript": "",
        "preRequestScript": "",
        "requestVariables": []
      },
      {
        "v": "13",
        "auth": {
          "token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiI4NjFiYTU3MC04ZThhLTQ2ZDAtYTNjMS00ZGMwZDA1ODdhYzAiLCJleHAiOjE3NDk3MzMzMzV9.tFYbd3bYk5XLY9ebbuvWokeVwE66gKlw7JTZb93Y1kM",
          "authType": "bearer",
          "authActive": true
        },
        "body": {
          "body": "{\n  \"count_in_stock\":65\n}\n",
          "contentType": "application/json"
        },
        "name": "update product stock",
        "method": "PUT",
        "params": [],
        "headers": [],
        "endpoint": "http://localhost:4000/api/v1/products/admin/09cfd9af-38e9-4fb4-bda3-0fafc287d79c/stock",
        "responses": {},
        "testScript": "",
        "preRequestScript": "",
        "requestVariables": []
      },
      {
        "v": "13",
        "auth": {
          "token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiI4NjFiYTU3MC04ZThhLTQ2ZDAtYTNjMS00ZGMwZDA1ODdhYzAiLCJleHAiOjE3NDk3ODA0OTR9.J7T0gR-tNK9dW1wKFlbsVImSawri9xECejVfNLuiMig",
          "authType": "bearer",
          "authActive": true
        },
        "body": {
          "body": "",
          "contentType": "application/json"
        },
        "name": "product status",
        "method": "PUT",
        "params": [],
        "headers": [],
        "endpoint": "http://localhost:4000/api/v1/products/admin/db798531-023a-4dfc-ad2e-49aceb4d0606/status",
        "responses": {},
        "testScript": "",
        "preRequestScript": "",
        "requestVariables": []
      },
      {
        "v": "13",
        "auth": {
          "token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiI4NjFiYTU3MC04ZThhLTQ2ZDAtYTNjMS00ZGMwZDA1ODdhYzAiLCJleHAiOjE3NDk3ODIyNzl9.ZlHgKb-m3ZPwLfmxrmTpk51gwxULeEYrr7wUjVw9C6c",
          "authType": "bearer",
          "authActive": true
        },
        "body": {
          "body": "{}",
          "contentType": "application/json"
        },
        "name": "get all admin product",
        "method": "GET",
        "params": [],
        "headers": [],
        "endpoint": "http://localhost:4000/api/v1/products/admin/all",
        "responses": {},
        "testScript": "",
        "preRequestScript": "",
        "requestVariables": []
      }
    ],
    "auth": {
      "authType": "inherit",
      "authActive": true
    },
    "headers": [],
    "_ref_id": "coll_mbtxdvo5_45457d5a-3920-4a4f-b617-978ced27cdb9"
  },
  {
    "v": 8,
    "name": "Orders",
    "folders": [],
    "requests": [
      {
        "v": "13",
        "auth": {
          "token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiI5MDM0YTUxNC0xYmFkLTRlYzAtOTcwNS02ZjEwYWViNTI0ZGEiLCJleHAiOjE3NDk3ODQ2MTJ9.r2Txm60n3k07KWyHYzBCDTDqnRbt2v-c2VMg155t4Cw",
          "authType": "bearer",
          "authActive": true
        },
        "body": {
          "body": "{\n  \"address_id\": \"f274b13f-f2da-4296-a6ec-32e1404e839b\"\n}\n",
          "contentType": "application/json"
        },
        "name": "create order",
        "method": "POST",
        "params": [],
        "headers": [],
        "endpoint": "http://localhost:4000/api/v1/orders/me/create",
        "responses": {},
        "testScript": "",
        "preRequestScript": "",
        "requestVariables": []
      },
      {
        "v": "13",
        "auth": {
          "token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiI4NjFiYTU3MC04ZThhLTQ2ZDAtYTNjMS00ZGMwZDA1ODdhYzAiLCJleHAiOjE3NDk3MzMzMzV9.tFYbd3bYk5XLY9ebbuvWokeVwE66gKlw7JTZb93Y1kM",
          "authType": "bearer",
          "authActive": true
        },
        "body": {
          "body": "{\n  \"order_status\": \"Delivered\",\n  \"payment_status\": \"Completed\",\n  \"user_id\": \"9034a514-1bad-4ec0-9705-6f10aeb524da\"\n}\n",
          "contentType": "application/json"
        },
        "name": "order status update",
        "method": "PUT",
        "params": [],
        "headers": [],
        "endpoint": "http://localhost:4000/api/v1/orders/admin/status/ORD-abe281aa607e408dbf356323518ebbf9",
        "responses": {},
        "testScript": "",
        "preRequestScript": "",
        "requestVariables": []
      },
      {
        "v": "13",
        "auth": {
          "token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiI5MDM0YTUxNC0xYmFkLTRlYzAtOTcwNS02ZjEwYWViNTI0ZGEiLCJleHAiOjE3NDk3ODQ2MTJ9.r2Txm60n3k07KWyHYzBCDTDqnRbt2v-c2VMg155t4Cw",
          "authType": "bearer",
          "authActive": true
        },
        "body": {
          "body": null,
          "contentType": null
        },
        "name": "get user orders",
        "method": "GET",
        "params": [],
        "headers": [],
        "endpoint": "http://localhost:4000/api/v1/orders/me/all",
        "responses": {},
        "testScript": "",
        "preRequestScript": "",
        "requestVariables": []
      }
    ],
    "auth": {
      "authType": "inherit",
      "authActive": true
    },
    "headers": [],
    "_ref_id": "coll_mbtxdvo5_6de6a3f1-312d-4515-a3df-3f0034ddc8b4"
  },
  {
    "v": 8,
    "name": "Cart",
    "folders": [],
    "requests": [
      {
        "v": "13",
        "auth": {
          "token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiI5MDM0YTUxNC0xYmFkLTRlYzAtOTcwNS02ZjEwYWViNTI0ZGEiLCJleHAiOjE3NDk3ODQ2MTJ9.r2Txm60n3k07KWyHYzBCDTDqnRbt2v-c2VMg155t4Cw",
          "authType": "bearer",
          "authActive": true
        },
        "body": {
          "body": "{\n  \"product_id\": \"db798531-023a-4dfc-ad2e-49aceb4d0606\",\n  \"quantity\": 3\n}\n\t",
          "contentType": "application/json"
        },
        "name": "add to cart",
        "method": "POST",
        "params": [],
        "headers": [],
        "endpoint": "http://localhost:4000/api/v1/cart/me/add",
        "responses": {},
        "testScript": "",
        "preRequestScript": "",
        "requestVariables": []
      },
      {
        "v": "13",
        "auth": {
          "token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiI5MDM0YTUxNC0xYmFkLTRlYzAtOTcwNS02ZjEwYWViNTI0ZGEiLCJleHAiOjE3NDk3ODQ2MTJ9.r2Txm60n3k07KWyHYzBCDTDqnRbt2v-c2VMg155t4Cw",
          "authType": "bearer",
          "authActive": true
        },
        "body": {
          "body": "",
          "contentType": "application/json"
        },
        "name": "remove from cart",
        "method": "DELETE",
        "params": [],
        "headers": [],
        "endpoint": "http://localhost:4000/api/v1/cart/me/db798531-023a-4dfc-ad2e-49aceb4d0606",
        "responses": {},
        "testScript": "",
        "preRequestScript": "",
        "requestVariables": []
      },
      {
        "v": "13",
        "auth": {
          "token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiI5MDM0YTUxNC0xYmFkLTRlYzAtOTcwNS02ZjEwYWViNTI0ZGEiLCJleHAiOjE3NDk3ODQ2MTJ9.r2Txm60n3k07KWyHYzBCDTDqnRbt2v-c2VMg155t4Cw",
          "authType": "bearer",
          "authActive": true
        },
        "body": {
          "body": "\n",
          "contentType": "application/json"
        },
        "name": "get uesr cart",
        "method": "GET",
        "params": [],
        "headers": [],
        "endpoint": "http://localhost:4000/api/v1/cart/me",
        "responses": {},
        "testScript": "",
        "preRequestScript": "",
        "requestVariables": []
      },
      {
        "v": "13",
        "auth": {
          "token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiI5MDM0YTUxNC0xYmFkLTRlYzAtOTcwNS02ZjEwYWViNTI0ZGEiLCJleHAiOjE3NDk3ODQ2MTJ9.r2Txm60n3k07KWyHYzBCDTDqnRbt2v-c2VMg155t4Cw",
          "authType": "bearer",
          "authActive": true
        },
        "body": {
          "body": null,
          "contentType": null
        },
        "name": "clear cart",
        "method": "DELETE",
        "params": [],
        "headers": [],
        "endpoint": "http://localhost:4000/api/v1/cart/me",
        "responses": {},
        "testScript": "",
        "preRequestScript": "",
        "requestVariables": []
      }
    ],
    "auth": {
      "authType": "inherit",
      "authActive": true
    },
    "headers": [],
    "_ref_id": "coll_mbtxdvo5_b58154db-2bff-44a3-af73-66ca639eb960"
  },
  {
    "v": 8,
    "name": "Address",
    "folders": [],
    "requests": [
      {
        "v": "13",
        "auth": {
          "token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiI5MDM0YTUxNC0xYmFkLTRlYzAtOTcwNS02ZjEwYWViNTI0ZGEiLCJleHAiOjE3NDk3Nzc1NjJ9.QVM3MyYP0mlacXLGWpgqNEr0EHwKll5a3ZttBDBzeO8",
          "authType": "bearer",
          "authActive": true
        },
        "body": {
          "body": "{\n  \"address_line1\": \"343,swastik flat\",\n  \"city\": \"surat\",\n  \"state\": \"Gujarat\",\n  \"pincode\": \"34007739\",\n  \"country\": \"India\",\n  \"mobile\": \"7778889993\"\n}\n",
          "contentType": "application/json"
        },
        "name": "create adddress",
        "method": "POST",
        "params": [],
        "headers": [],
        "endpoint": "http://localhost:4000/api/v1/addresses/me",
        "responses": {},
        "testScript": "",
        "preRequestScript": "",
        "requestVariables": []
      },
      {
        "v": "13",
        "auth": {
          "token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiI5MDM0YTUxNC0xYmFkLTRlYzAtOTcwNS02ZjEwYWViNTI0ZGEiLCJleHAiOjE3NDk3Nzc1NjJ9.QVM3MyYP0mlacXLGWpgqNEr0EHwKll5a3ZttBDBzeO8",
          "authType": "bearer",
          "authActive": true
        },
        "body": {
          "body": null,
          "contentType": null
        },
        "name": "get address by id",
        "method": "GET",
        "params": [],
        "headers": [],
        "endpoint": "http://localhost:4000/api/v1/addresses/me/f66aae1e-267e-4d6b-a9b6-587620e0eeef",
        "responses": {},
        "testScript": "",
        "preRequestScript": "",
        "requestVariables": []
      },
      {
        "v": "13",
        "auth": {
          "token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiI4NjFiYTU3MC04ZThhLTQ2ZDAtYTNjMS00ZGMwZDA1ODdhYzAiLCJleHAiOjE3NDk2NTYxMDJ9.KaLCgFC9RycNu1TcRNDP33DyIZia8r7BpDxM--Nm2Zk",
          "authType": "bearer",
          "authActive": true
        },
        "body": {
          "body": "{\n  \"address_line1\": \"455, swastik flat\",\n  \"city\": \"rajkot\",\n  \"state\": \"Gujarat\",\n  \"pincode\": \"388815\",\n  \"country\": \"India\",\n  \"mobile\": \"77452539912\"\n\n}\n",
          "contentType": "application/json"
        },
        "name": "update address",
        "method": "PUT",
        "params": [],
        "headers": [],
        "endpoint": "http://localhost:4000/api/v1/addresses/me/f66aae1e-267e-4d6b-a9b6-587620e0eeef",
        "responses": {},
        "testScript": "",
        "preRequestScript": "",
        "requestVariables": []
      },
      {
        "v": "13",
        "auth": {
          "token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiI5MDM0YTUxNC0xYmFkLTRlYzAtOTcwNS02ZjEwYWViNTI0ZGEiLCJleHAiOjE3NDk3Nzc1NjJ9.QVM3MyYP0mlacXLGWpgqNEr0EHwKll5a3ZttBDBzeO8",
          "authType": "bearer",
          "authActive": true
        },
        "body": {
          "body": null,
          "contentType": null
        },
        "name": "get all addresses of user",
        "method": "GET",
        "params": [],
        "headers": [],
        "endpoint": "http://localhost:4000/api/v1/addresses/me/all",
        "responses": {},
        "testScript": "",
        "preRequestScript": "",
        "requestVariables": []
      },
      {
        "v": "13",
        "auth": {
          "token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiI5MDM0YTUxNC0xYmFkLTRlYzAtOTcwNS02ZjEwYWViNTI0ZGEiLCJleHAiOjE3NDk3Nzc1NjJ9.QVM3MyYP0mlacXLGWpgqNEr0EHwKll5a3ZttBDBzeO8",
          "authType": "bearer",
          "authActive": true
        },
        "body": {
          "body": null,
          "contentType": null
        },
        "name": "delete user address",
        "method": "DELETE",
        "params": [],
        "headers": [
          {
            "key": "Authorization",
            "value": "Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJfaWQiOiI2N2YzMWFmNTM3NjM0NmFiZjRmNWM3NDciLCJlbWFpbCI6ImhpbWFuc2h1LmNoYXZkYUByYXBpZG9wcy5jbyIsImZ1bGxOYW1lIjoiSGltYW5zaHUiLCJyb2xlIjoiQ1VTVE9NRVIiLCJzdGF0dXMiOiJBY3RpdmUiLCJpYXQiOjE3NDM5OTIwMzMsImV4cCI6MTc0Mzk5MzIzM30.W4Gbo5lHi3uEgh0fOUt8xahD3uaEqtnJoprelda1kpk",
            "active": true,
            "description": ""
          }
        ],
        "endpoint": "http://localhost:4000/api/v1/addresses/me/b0a1b979-96f1-4af8-b08d-5f919c9b6923",
        "responses": {},
        "testScript": "",
        "preRequestScript": "",
        "requestVariables": []
      },
      {
        "v": "13",
        "auth": {
          "token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiI4NjFiYTU3MC04ZThhLTQ2ZDAtYTNjMS00ZGMwZDA1ODdhYzAiLCJleHAiOjE3NDk2NTYxMDJ9.KaLCgFC9RycNu1TcRNDP33DyIZia8r7BpDxM--Nm2Zk",
          "authType": "bearer",
          "authActive": true
        },
        "body": {
          "body": null,
          "contentType": null
        },
        "name": "select address",
        "method": "PUT",
        "params": [],
        "headers": [],
        "endpoint": "http://localhost:4000/api/v1/addresses/me/select/df00c604-db6a-4d34-b3ca-66cdeb841834",
        "responses": {},
        "testScript": "",
        "preRequestScript": "",
        "requestVariables": []
      }
    ],
    "auth": {
      "authType": "inherit",
      "authActive": true
    },
    "headers": [],
    "_ref_id": "coll_mbtxdvo5_81b35e4d-8917-4da5-bf04-ea366a161154"
  },
  {
    "v": 8,
    "name": "user",
    "folders": [],
    "requests": [
      {
        "v": "13",
        "auth": {
          "token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiI5MDM0YTUxNC0xYmFkLTRlYzAtOTcwNS02ZjEwYWViNTI0ZGEiLCJleHAiOjE3NDk3Nzc1NjJ9.QVM3MyYP0mlacXLGWpgqNEr0EHwKll5a3ZttBDBzeO8",
          "authType": "bearer",
          "authActive": true
        },
        "body": {
          "body": "{\n  \"full_name\": \"raju shrivstav\",\n  \"mobile\": \"8483732948\"\n}\n",
          "contentType": "application/json"
        },
        "name": "Update Customer info",
        "method": "PUT",
        "params": [],
        "headers": [],
        "endpoint": "http://localhost:4000/api/v1/user/me/update",
        "responses": {},
        "testScript": "",
        "preRequestScript": "",
        "requestVariables": []
      },
      {
        "v": "13",
        "auth": {
          "token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiI5MDM0YTUxNC0xYmFkLTRlYzAtOTcwNS02ZjEwYWViNTI0ZGEiLCJleHAiOjE3NDk3Nzc1NjJ9.QVM3MyYP0mlacXLGWpgqNEr0EHwKll5a3ZttBDBzeO8",
          "authType": "bearer",
          "authActive": true
        },
        "body": {
          "body": null,
          "contentType": null
        },
        "name": "get current user",
        "method": "GET",
        "params": [],
        "headers": [],
        "endpoint": "http://localhost:4000/api/v1/user/me/get",
        "responses": {},
        "testScript": "",
        "preRequestScript": "",
        "requestVariables": []
      },
      {
        "v": "13",
        "auth": {
          "token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiI5MDM0YTUxNC0xYmFkLTRlYzAtOTcwNS02ZjEwYWViNTI0ZGEiLCJleHAiOjE3NDk3Nzg1MDN9.X4HiUAvZRxCBUzSoQ2ezKbK-KSRolauSR2nJsFizLB8",
          "authType": "bearer",
          "authActive": true
        },
        "body": {
          "body": "{\n  \"status\": \"Inactive\",\n  \"user_id\": \"da02047b-f941-4fc8-8e21-656563cbff32\"\n}\n",
          "contentType": "application/json"
        },
        "name": "change user status",
        "method": "PUT",
        "params": [],
        "headers": [],
        "endpoint": "http://localhost:4000/api/v1/user/admin/status",
        "responses": {},
        "testScript": "",
        "preRequestScript": "",
        "requestVariables": []
      }
    ],
    "auth": {
      "authType": "inherit",
      "authActive": true
    },
    "headers": [],
    "_ref_id": "coll_mbtxdvo5_020b175b-0d55-4631-b1d0-474d6ee0c9eb"
  }
]