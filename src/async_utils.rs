use std::future::Future;

use futures::{pin_mut, Stream, StreamExt};

pub async fn try_for_each<T, E, F>(stream: impl Stream<Item = T>, mut f: impl FnMut(T) -> F) -> Result<(), E>
where
	F: Future<Output = Result<(), E>>,
{
	pin_mut!(stream);

	while let Some(item) = stream.next().await {
		f(item).await?
	}

	Ok(())
}
