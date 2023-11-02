Там, где не показан ответ запроса, он стандартный
```json
{
    "response": null,
    "success": true
}
```
Там, где не описаны тела запросов- их нет


1. Получить всех сотрудников
```
GET /employee/
```
ответ: 
```json
{
    "response": [
        {
            "email": "lol",
            "id": "653176ed-43e5-4bf8-b50d-703a8d317efd",
            "isManager": true,
            "name": "name",
            "secondName": "sec",
            "surname": "sss"
        }
    ],
    "success": true
}
```


1.1 Получить конкретного
```
GET /employee/:id
```

ответ: 
```json
{
    "response": {
        "email": "lol",
        "id": "653176ed-43e5-4bf8-b50d-703a8d317efd",
        "isManager": true,
        "name": "name",
        "secondName": "sec",
        "surname": "sss"
    },
    "success": true
}
```

2. Создать сотрудника
 
```
POST /employee/new 
```
тело запроса:

```json
{
    "name": "name",
    "secondName": "sec",
    "surname": "sss",
    "email": "lol",
    "isManager": true
}
```

3. Удалить сотрудника


```
DELETE /employee/remote 
```

тело запроса:

```json
{
    "id": "653176ed-43e5-4bf8-b50d-703a8d317efd",
}
```


4. обновить данные 

```
PATCH /employee/:id
```

Присылать струкруру целиком, а не только обновленные поля
```json
{
    "name": "name",
    "secondName": "sec",
    "surname": "sss",
    "email": "lol",
    "isManager": true
}
```