use async_std::prelude::*;
use serde::de::DeserializeOwned;

pub async fn send_as_json<S, P>(outbound: &mut S, packet: &P) -> anyhow::Result<()>
where
    S: async_std::io::Write + Unpin,
    P: serde::Serialize,
{
    let mut json = serde_json::to_string(&packet)?;
    json.push('\n');
    outbound.write_all(json.as_bytes()).await?;
    Ok(())
}

pub fn receive_as_json<S, P>(inbound: S) -> impl Stream<Item = anyhow::Result<P>>
where
    S: async_std::io::BufRead + Unpin,
    P: DeserializeOwned,
{
    inbound.lines().map(|lines_result| -> anyhow::Result<P> {
        let line = lines_result?;
        let parsed = serde_json::from_str::<P>(&line)?;
        Ok(parsed)
    })
}
