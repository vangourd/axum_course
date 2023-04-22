use crate::model::{ModelController, Ticket, TicketForCreate};
use axum::{extract::State, Json};
use crate::Result;

// region:      --- REST handlers
// endregion:   --- REST handlers

async fn create_ticket(
    State(mc): State<ModelController>,
    Json(ticket_fc): Json<TicketForCreate>,
) -> Result<Json<Ticket>> {
    println!("->> {:<12} - create_ticket","HANDLER");

    let ticket = mc.create_ticket(ticket_fc).await?;
    
    Ok(Json(ticket))
}

async fn list_tickets (
    State(mc): State<ModelController>,
) -> <Result<Json<Ticket>>> {
    println!("->> {:<12} - list_tickets","HANDLER");

    let tickets = mc.list_tickets().await()?;

    Ok(Json(tickets))

}