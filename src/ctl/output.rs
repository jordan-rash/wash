use std::collections::HashMap;

use anyhow::{bail, Result};
use serde_json::json;
use term_table::{row::Row, table_cell::*, Table};
use wash_lib::cli::CommandOutput;
use wash_lib::id::{ModuleId, ServiceId};
use wasmcloud_control_interface::*;

use crate::util::format_optional;

pub(crate) fn get_hosts_output(hosts: Vec<Host>) -> CommandOutput {
    let mut map = HashMap::new();
    map.insert("hosts".to_string(), json!(hosts));
    CommandOutput::new(hosts_table(hosts), map)
}

pub(crate) fn get_host_inventory_output(inv: HostInventory) -> CommandOutput {
    let mut map = HashMap::new();
    map.insert("inventory".to_string(), json!(inv));
    CommandOutput::new(host_inventory_table(inv), map)
}

pub(crate) fn get_claims_output(claims: GetClaimsResponse) -> CommandOutput {
    let mut map = HashMap::new();
    map.insert("claims".to_string(), json!(claims));
    CommandOutput::new(claims_table(claims), map)
}

pub(crate) fn link_del_output(
    actor_id: &ModuleId,
    contract_id: &str,
    link_name: &str,
    failure: Option<String>,
) -> Result<CommandOutput> {
    match failure {
        None => {
            let mut map = HashMap::new();
            map.insert("actor_id".to_string(), json!(actor_id));
            map.insert("contract_id".to_string(), json!(contract_id));
            map.insert("link_name".to_string(), json!(link_name));
            Ok(CommandOutput::new(
                format!(
                    "Deleted link for {} on {} ({}) successfully",
                    actor_id, contract_id, link_name
                ),
                map,
            ))
        }
        Some(f) => bail!("Error deleting link: {}", f),
    }
}

pub(crate) fn link_put_output(
    actor_id: &ModuleId,
    provider_id: &ServiceId,
    failure: Option<String>,
) -> Result<CommandOutput> {
    match failure {
        None => {
            let mut map = HashMap::new();
            map.insert("actor_id".to_string(), json!(actor_id));
            map.insert("provider_id".to_string(), json!(provider_id));
            Ok(CommandOutput::new(
                format!(
                    "Published link ({}) <-> ({}) successfully",
                    actor_id, provider_id
                ),
                map,
            ))
        }
        Some(f) => bail!("Error advertising link: {}", f),
    }
}

pub(crate) fn link_query_output(list: LinkDefinitionList) -> CommandOutput {
    let mut map = HashMap::new();
    map.insert("links".to_string(), json!(list.links));
    CommandOutput::new(links_table(list), map)
}

pub(crate) fn apply_manifest_output(results: Vec<String>) -> CommandOutput {
    let mut map = HashMap::new();
    map.insert("results".to_string(), json!(results));

    CommandOutput::new(
        format!("\nManifest application results:\n{}", results.join("\n")),
        map,
    )
}

/// Helper function to transform a LinkDefinitionList into a table string for printing
pub(crate) fn links_table(list: LinkDefinitionList) -> String {
    let mut table = Table::new();
    crate::util::configure_table_style(&mut table);

    table.add_row(Row::new(vec![
        TableCell::new_with_alignment("Actor ID", 1, Alignment::Left),
        TableCell::new_with_alignment("Provider ID", 1, Alignment::Left),
        TableCell::new_with_alignment("Contract ID", 1, Alignment::Left),
        TableCell::new_with_alignment("Link Name", 1, Alignment::Left),
    ]));

    list.links.iter().for_each(|l| {
        table.add_row(Row::new(vec![
            TableCell::new_with_alignment(l.actor_id.clone(), 1, Alignment::Left),
            TableCell::new_with_alignment(l.provider_id.clone(), 1, Alignment::Left),
            TableCell::new_with_alignment(l.contract_id.clone(), 1, Alignment::Left),
            TableCell::new_with_alignment(l.link_name.clone(), 1, Alignment::Left),
        ]))
    });

    table.render()
}

/// Helper function to transform a Host list into a table string for printing
pub(crate) fn hosts_table(hosts: Vec<Host>) -> String {
    let mut table = Table::new();
    crate::util::configure_table_style(&mut table);

    table.add_row(Row::new(vec![
        TableCell::new_with_alignment("Host ID", 1, Alignment::Left),
        TableCell::new_with_alignment("Uptime (seconds)", 1, Alignment::Left),
    ]));
    hosts.iter().for_each(|h| {
        table.add_row(Row::new(vec![
            TableCell::new_with_alignment(h.id.clone(), 1, Alignment::Left),
            TableCell::new_with_alignment(format!("{}", h.uptime_seconds), 1, Alignment::Left),
        ]))
    });

    table.render()
}

/// Helper function to transform a HostInventory into a table string for printing
pub(crate) fn host_inventory_table(inv: HostInventory) -> String {
    let mut table = Table::new();
    crate::util::configure_table_style(&mut table);

    table.add_row(Row::new(vec![TableCell::new_with_alignment(
        format!("Host Inventory ({})", inv.host_id),
        4,
        Alignment::Center,
    )]));

    if !inv.labels.is_empty() {
        table.add_row(Row::new(vec![TableCell::new_with_alignment(
            "",
            4,
            Alignment::Center,
        )]));
        inv.labels.iter().for_each(|(k, v)| {
            table.add_row(Row::new(vec![
                TableCell::new_with_alignment(k, 2, Alignment::Left),
                TableCell::new_with_alignment(v, 2, Alignment::Left),
            ]))
        });
    } else {
        table.add_row(Row::new(vec![TableCell::new_with_alignment(
            "No labels present",
            4,
            Alignment::Center,
        )]));
    }

    table.add_row(Row::new(vec![TableCell::new_with_alignment(
        "",
        4,
        Alignment::Center,
    )]));
    if !inv.actors.is_empty() {
        table.add_row(Row::new(vec![
            TableCell::new_with_alignment("Actor ID", 1, Alignment::Left),
            TableCell::new_with_alignment("Name", 1, Alignment::Left),
            TableCell::new_with_alignment("Image Reference", 2, Alignment::Left),
        ]));
        inv.actors.iter().for_each(|a| {
            let a = a.clone();
            table.add_row(Row::new(vec![
                TableCell::new_with_alignment(a.id, 1, Alignment::Left),
                TableCell::new_with_alignment(format_optional(a.name), 1, Alignment::Left),
                TableCell::new_with_alignment(format_optional(a.image_ref), 2, Alignment::Left),
            ]))
        });
    } else {
        table.add_row(Row::new(vec![TableCell::new_with_alignment(
            "No actors found",
            4,
            Alignment::Left,
        )]));
    }
    table.add_row(Row::new(vec![TableCell::new_with_alignment(
        "",
        4,
        Alignment::Left,
    )]));
    if !inv.providers.is_empty() {
        table.add_row(Row::new(vec![
            TableCell::new_with_alignment("Provider ID", 1, Alignment::Left),
            TableCell::new_with_alignment("Name", 1, Alignment::Left),
            TableCell::new_with_alignment("Link Name", 1, Alignment::Left),
            TableCell::new_with_alignment("Image Reference", 1, Alignment::Left),
        ]));
        inv.providers.iter().for_each(|p| {
            let p = p.clone();
            table.add_row(Row::new(vec![
                TableCell::new_with_alignment(p.id, 1, Alignment::Left),
                TableCell::new_with_alignment(format_optional(p.name), 1, Alignment::Left),
                TableCell::new_with_alignment(p.link_name, 1, Alignment::Left),
                TableCell::new_with_alignment(format_optional(p.image_ref), 1, Alignment::Left),
            ]))
        });
    } else {
        table.add_row(Row::new(vec![TableCell::new_with_alignment(
            "No providers found",
            4,
            Alignment::Left,
        )]));
    }

    table.render()
}

/// Helper function to transform a ClaimsList into a table string for printing
pub(crate) fn claims_table(list: GetClaimsResponse) -> String {
    let mut table = Table::new();
    crate::util::configure_table_style(&mut table);

    table.add_row(Row::new(vec![TableCell::new_with_alignment(
        "Claims",
        2,
        Alignment::Center,
    )]));

    list.claims.iter().for_each(|c| {
        table.add_row(Row::new(vec![
            TableCell::new_with_alignment("Issuer", 1, Alignment::Left),
            TableCell::new_with_alignment(
                c.get("iss").unwrap_or(&"".to_string()),
                1,
                Alignment::Left,
            ),
        ]));
        table.add_row(Row::new(vec![
            TableCell::new_with_alignment("Subject", 1, Alignment::Left),
            TableCell::new_with_alignment(
                c.get("sub").unwrap_or(&"".to_string()),
                1,
                Alignment::Left,
            ),
        ]));
        table.add_row(Row::new(vec![
            TableCell::new_with_alignment("Capabilities", 1, Alignment::Left),
            TableCell::new_with_alignment(
                c.get("caps").unwrap_or(&"".to_string()),
                1,
                Alignment::Left,
            ),
        ]));
        table.add_row(Row::new(vec![
            TableCell::new_with_alignment("Version", 1, Alignment::Left),
            TableCell::new_with_alignment(
                c.get("version").unwrap_or(&"".to_string()),
                1,
                Alignment::Left,
            ),
        ]));
        table.add_row(Row::new(vec![
            TableCell::new_with_alignment("Revision", 1, Alignment::Left),
            TableCell::new_with_alignment(
                c.get("rev").unwrap_or(&"".to_string()),
                1,
                Alignment::Left,
            ),
        ]));
        table.add_row(Row::new(vec![TableCell::new_with_alignment(
            String::new(),
            2,
            Alignment::Center,
        )]));
    });

    table.render()
}
