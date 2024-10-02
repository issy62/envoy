use libsql::errors::Error as SqlError;
use libsql::Connection as SqlConnection;

#[derive(Default)]
pub struct LoggerContext {
    from: String,
    to: String,
    message: String,
    operator: String,
    timestamp: String,
    response: String,
}

impl LoggerContext {
    pub fn new(
        from: &str,
        to: &str,
        message: &str,
        operator: &str,
        timestamp: &str,
        response: &str,
    ) -> Self {
        Self {
            from: from.to_string(),
            to: to.to_string(),
            message: message.to_string(),
            operator: operator.to_string(),
            timestamp: timestamp.to_string(),
            response: response.to_string(),
        }
    }
}

pub async fn log_to_turso(conn: &SqlConnection, log_ctx: LoggerContext) -> Result<u64, SqlError> {
    let query = "INSERT INTO msg_log (msg_from, msg_to, msg_body, msg_operator, msg_timestamp, msg_resp) VALUES (?1, ?2, ?3, ?4, ?5, ?6)";

    match conn
        .execute(
            &query,
            (
                log_ctx.from,
                log_ctx.to,
                log_ctx.message,
                log_ctx.operator,
                log_ctx.timestamp,
                log_ctx.response,
            ),
        )
        .await
    {
        Err(e) => return Err(e),
        Ok(r) => {
            return Ok(r);
        }
    }
}

