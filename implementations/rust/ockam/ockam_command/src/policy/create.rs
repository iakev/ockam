use crate::node::get_node_name;
use crate::policy::policy_path;
use crate::util::{node_rpc, parse_node_name, Rpc};
use crate::CommandGlobalOpts;
use clap::Args;
use ockam::Context;
use ockam_abac::{Action, Expr, Resource};
use ockam_api::nodes::models::policy::Policy;
use ockam_core::api::Request;

#[derive(Clone, Debug, Args)]
pub struct CreateCommand {
    #[arg(long, display_order = 900, id = "NODE_NAME")]
    at: Option<String>,

    #[arg(short, long)]
    resource: Resource,

    #[arg(short, long, default_value = "handle_message")]
    action: Action,

    #[arg(short, long)]
    expression: Expr,
}

impl CreateCommand {
    pub fn run(self, options: CommandGlobalOpts) {
        node_rpc(rpc, (options, self));
    }
}

async fn rpc(
    mut ctx: Context,
    (opts, cmd): (CommandGlobalOpts, CreateCommand),
) -> miette::Result<()> {
    run_impl(&mut ctx, opts, cmd).await
}

async fn run_impl(
    ctx: &mut Context,
    opts: CommandGlobalOpts,
    cmd: CreateCommand,
) -> miette::Result<()> {
    let at = get_node_name(&opts.state, &cmd.at);
    let node_name = parse_node_name(&at)?;
    let bdy = Policy::new(cmd.expression);
    let req = Request::post(policy_path(&cmd.resource, &cmd.action)).body(bdy);
    let mut rpc = Rpc::background(ctx, &opts, &node_name)?;
    rpc.request(req).await?;
    rpc.is_ok()?;
    Ok(())
}
