namespace rs mini.redis

struct KVPair {
    1: required string Key,
    2: required string Value,
}

struct GetValueRequest {
    1: required string Key,
}

struct GetValueResponse {
    1: required string Value,
    2: required string Error,
}

struct SetValueRequest {
    1: required string Key,
    2: required string Value,
    3: optional i32 ExpireSeconds,
}

struct SetValueResponse {
    1: required string Error,
}

struct DeleteValueRequest {
    1: required string Key,
}

struct DeleteValueResponse {
    1: required string Error,
}

struct PingResponse {
    1: required string Pong,
}

service MiniRedisService {
    GetValueResponse GetValue(1: GetValueRequest request),
    SetValueResponse SetValue(1: SetValueRequest request),
    DeleteValueResponse DeleteValue(1: DeleteValueRequest request),
    PingResponse Ping(),
}