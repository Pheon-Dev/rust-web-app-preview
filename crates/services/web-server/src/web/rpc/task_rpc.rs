use crate::web::rpc::{DataResult, ParamsForCreate, ParamsForUpdate, ParamsIded};
use crate::web::Result;
use lib_core::ctx::Ctx;
use lib_core::model::task::{Task, TaskBmc, TaskForCreate, TaskForUpdate};
use lib_core::model::ModelManager;

pub async fn create_task(
	ctx: Ctx,
	mm: ModelManager,
	params: ParamsForCreate<TaskForCreate>,
) -> Result<DataResult<Task>> {
	let ParamsForCreate { data } = params;

	let id = TaskBmc::create(&ctx, &mm, data).await?;
	let task = TaskBmc::get(&ctx, &mm, id).await?;

	Ok(DataResult::new(task))
}

pub async fn list_tasks(
	ctx: Ctx,
	mm: ModelManager,
) -> Result<DataResult<Vec<Task>>> {
	let tasks = TaskBmc::list(&ctx, &mm).await?;

	Ok(DataResult::new(tasks))
}

pub async fn update_task(
	ctx: Ctx,
	mm: ModelManager,
	params: ParamsForUpdate<TaskForUpdate>,
) -> Result<DataResult<Task>> {
	let ParamsForUpdate { id, data } = params;

	TaskBmc::update(&ctx, &mm, id, data).await?;

	let task = TaskBmc::get(&ctx, &mm, id).await?;

	Ok(DataResult::new(task))
}

pub async fn show_task(
	ctx: Ctx,
	mm: ModelManager,
	params: ParamsIded,
) -> Result<DataResult<Task>> {
	let ParamsIded { id } = params;

	let task = TaskBmc::get(&ctx, &mm, id).await?;
	TaskBmc::show(&ctx, &mm, id).await?;

	Ok(DataResult::new(task))
}

pub async fn delete_task(
	ctx: Ctx,
	mm: ModelManager,
	params: ParamsIded,
) -> Result<DataResult<Task>> {
	let ParamsIded { id } = params;

	let task = TaskBmc::get(&ctx, &mm, id).await?;
	TaskBmc::delete(&ctx, &mm, id).await?;

	Ok(DataResult::new(task))
}
