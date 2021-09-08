use bcrypt::DEFAULT_COST;

use crate::error::Result;

// consume password value to make it unusable
pub async fn hash_password(password: String) -> Result<String> {
    let (send, recv) = tokio::sync::oneshot::channel();
    rayon::spawn(move || {
        let result = bcrypt::hash(password, DEFAULT_COST);
        let _ = send.send(result);
    });
    Ok(recv.await??)
}

pub async fn verify_password(password: String, hash: String) -> Result<bool> {
    let (send, recv) = tokio::sync::oneshot::channel();
    rayon::spawn(move || {
        let result = bcrypt::verify(password, &hash);
        let _ = send.send(result);
    });
    Ok(recv.await??)
}
