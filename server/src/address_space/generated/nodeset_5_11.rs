// This file was autogenerated from Opc.Ua.NodeSet2.Part5.xml by tools/schema/gen_address_space.js
// DO NOT EDIT THIS FILE

#[allow(unused_imports)]
use std::str::FromStr;

#[allow(unused_imports)]
use opcua_types::{
    node_id::NodeId,
    data_value::DataValue,
    variant::Variant, 
    extension_object::ExtensionObject, 
    string::UAString,
    basic_types::LocalizedText,
    service_types::{
        Argument
    },
    node_ids::*
};
#[allow(unused_imports)]
use crate::address_space::types::*;

#[allow(unused_variables)]
pub fn populate_address_space(address_space: &mut AddressSpace) {
    add_method_1(address_space);
    add_method_2(address_space);
    add_method_3(address_space);
    add_method_4(address_space);
    add_method_5(address_space);
    add_method_6(address_space);
    add_method_7(address_space);
    add_method_8(address_space);
    add_method_9(address_space);
    add_method_10(address_space);
    add_method_11(address_space);
    add_method_12(address_space);
}

fn add_method_1(address_space: &mut AddressSpace) {
    // Method
    let browse_name = "GetPosition";
    let display_name = "GetPosition";
    let description = "";
    let node_id = NodeId::new(0, 11639);
    let node = Method::new(&node_id, browse_name, display_name, description, false, false, false);
    address_space.insert(node, Some(&[
        (&NodeId::new(0, 11624), ReferenceTypeId::Organizes, ReferenceDirection::Inverse),
        (&NodeId::new(0, 11640), ReferenceTypeId::HasProperty, ReferenceDirection::Forward),
        (&NodeId::new(0, 11641), ReferenceTypeId::HasProperty, ReferenceDirection::Forward),
        (&NodeId::new(0, 78), ReferenceTypeId::HasModellingRule, ReferenceDirection::Forward),
        (&NodeId::new(0, 11624), ReferenceTypeId::HasComponent, ReferenceDirection::Inverse),
    ]));
}

fn add_method_2(address_space: &mut AddressSpace) {
    // Method
    let browse_name = "SetPosition";
    let display_name = "SetPosition";
    let description = "";
    let node_id = NodeId::new(0, 11642);
    let node = Method::new(&node_id, browse_name, display_name, description, false, false, false);
    address_space.insert(node, Some(&[
        (&NodeId::new(0, 11624), ReferenceTypeId::Organizes, ReferenceDirection::Inverse),
        (&NodeId::new(0, 11643), ReferenceTypeId::HasProperty, ReferenceDirection::Forward),
        (&NodeId::new(0, 78), ReferenceTypeId::HasModellingRule, ReferenceDirection::Forward),
        (&NodeId::new(0, 11624), ReferenceTypeId::HasComponent, ReferenceDirection::Inverse),
    ]));
}

fn add_method_3(address_space: &mut AddressSpace) {
    // Method
    let browse_name = "Open";
    let display_name = "Open";
    let description = "";
    let node_id = NodeId::new(0, 11680);
    let node = Method::new(&node_id, browse_name, display_name, description, false, false, false);
    address_space.insert(node, Some(&[
        (&NodeId::new(0, 11675), ReferenceTypeId::Organizes, ReferenceDirection::Inverse),
        (&NodeId::new(0, 11681), ReferenceTypeId::HasProperty, ReferenceDirection::Forward),
        (&NodeId::new(0, 11682), ReferenceTypeId::HasProperty, ReferenceDirection::Forward),
        (&NodeId::new(0, 78), ReferenceTypeId::HasModellingRule, ReferenceDirection::Forward),
        (&NodeId::new(0, 11675), ReferenceTypeId::HasComponent, ReferenceDirection::Inverse),
    ]));
}

fn add_method_4(address_space: &mut AddressSpace) {
    // Method
    let browse_name = "Close";
    let display_name = "Close";
    let description = "";
    let node_id = NodeId::new(0, 11683);
    let node = Method::new(&node_id, browse_name, display_name, description, false, false, false);
    address_space.insert(node, Some(&[
        (&NodeId::new(0, 11675), ReferenceTypeId::Organizes, ReferenceDirection::Inverse),
        (&NodeId::new(0, 11684), ReferenceTypeId::HasProperty, ReferenceDirection::Forward),
        (&NodeId::new(0, 78), ReferenceTypeId::HasModellingRule, ReferenceDirection::Forward),
        (&NodeId::new(0, 11675), ReferenceTypeId::HasComponent, ReferenceDirection::Inverse),
    ]));
}

fn add_method_5(address_space: &mut AddressSpace) {
    // Method
    let browse_name = "Read";
    let display_name = "Read";
    let description = "";
    let node_id = NodeId::new(0, 11685);
    let node = Method::new(&node_id, browse_name, display_name, description, false, false, false);
    address_space.insert(node, Some(&[
        (&NodeId::new(0, 11675), ReferenceTypeId::Organizes, ReferenceDirection::Inverse),
        (&NodeId::new(0, 11686), ReferenceTypeId::HasProperty, ReferenceDirection::Forward),
        (&NodeId::new(0, 11687), ReferenceTypeId::HasProperty, ReferenceDirection::Forward),
        (&NodeId::new(0, 78), ReferenceTypeId::HasModellingRule, ReferenceDirection::Forward),
        (&NodeId::new(0, 11675), ReferenceTypeId::HasComponent, ReferenceDirection::Inverse),
    ]));
}

fn add_method_6(address_space: &mut AddressSpace) {
    // Method
    let browse_name = "Write";
    let display_name = "Write";
    let description = "";
    let node_id = NodeId::new(0, 11688);
    let node = Method::new(&node_id, browse_name, display_name, description, false, false, false);
    address_space.insert(node, Some(&[
        (&NodeId::new(0, 11675), ReferenceTypeId::Organizes, ReferenceDirection::Inverse),
        (&NodeId::new(0, 11689), ReferenceTypeId::HasProperty, ReferenceDirection::Forward),
        (&NodeId::new(0, 78), ReferenceTypeId::HasModellingRule, ReferenceDirection::Forward),
        (&NodeId::new(0, 11675), ReferenceTypeId::HasComponent, ReferenceDirection::Inverse),
    ]));
}

fn add_method_7(address_space: &mut AddressSpace) {
    // Method
    let browse_name = "GetPosition";
    let display_name = "GetPosition";
    let description = "";
    let node_id = NodeId::new(0, 11690);
    let node = Method::new(&node_id, browse_name, display_name, description, false, false, false);
    address_space.insert(node, Some(&[
        (&NodeId::new(0, 11675), ReferenceTypeId::Organizes, ReferenceDirection::Inverse),
        (&NodeId::new(0, 11691), ReferenceTypeId::HasProperty, ReferenceDirection::Forward),
        (&NodeId::new(0, 11692), ReferenceTypeId::HasProperty, ReferenceDirection::Forward),
        (&NodeId::new(0, 78), ReferenceTypeId::HasModellingRule, ReferenceDirection::Forward),
        (&NodeId::new(0, 11675), ReferenceTypeId::HasComponent, ReferenceDirection::Inverse),
    ]));
}

fn add_method_8(address_space: &mut AddressSpace) {
    // Method
    let browse_name = "SetPosition";
    let display_name = "SetPosition";
    let description = "";
    let node_id = NodeId::new(0, 11693);
    let node = Method::new(&node_id, browse_name, display_name, description, false, false, false);
    address_space.insert(node, Some(&[
        (&NodeId::new(0, 11675), ReferenceTypeId::Organizes, ReferenceDirection::Inverse),
        (&NodeId::new(0, 11694), ReferenceTypeId::HasProperty, ReferenceDirection::Forward),
        (&NodeId::new(0, 78), ReferenceTypeId::HasModellingRule, ReferenceDirection::Forward),
        (&NodeId::new(0, 11675), ReferenceTypeId::HasComponent, ReferenceDirection::Inverse),
    ]));
}

fn add_method_9(address_space: &mut AddressSpace) {
    // Method
    let browse_name = "GetMonitoredItems";
    let display_name = "GetMonitoredItems";
    let description = "";
    let node_id = NodeId::new(0, 11492);
    let node = Method::new(&node_id, browse_name, display_name, description, false, false, false);
    address_space.insert(node, Some(&[
        (&NodeId::new(0, 2253), ReferenceTypeId::Organizes, ReferenceDirection::Inverse),
        (&NodeId::new(0, 11493), ReferenceTypeId::HasProperty, ReferenceDirection::Forward),
        (&NodeId::new(0, 11494), ReferenceTypeId::HasProperty, ReferenceDirection::Forward),
        (&NodeId::new(0, 2253), ReferenceTypeId::HasComponent, ReferenceDirection::Inverse),
    ]));
}

fn add_method_10(address_space: &mut AddressSpace) {
    // Method
    let browse_name = "ResendData";
    let display_name = "ResendData";
    let description = "";
    let node_id = NodeId::new(0, 12873);
    let node = Method::new(&node_id, browse_name, display_name, description, false, false, false);
    address_space.insert(node, Some(&[
        (&NodeId::new(0, 2253), ReferenceTypeId::Organizes, ReferenceDirection::Inverse),
        (&NodeId::new(0, 12874), ReferenceTypeId::HasProperty, ReferenceDirection::Forward),
        (&NodeId::new(0, 2253), ReferenceTypeId::HasComponent, ReferenceDirection::Inverse),
    ]));
}

fn add_method_11(address_space: &mut AddressSpace) {
    // Method
    let browse_name = "SetSubscriptionDurable";
    let display_name = "SetSubscriptionDurable";
    let description = "";
    let node_id = NodeId::new(0, 12749);
    let node = Method::new(&node_id, browse_name, display_name, description, false, false, false);
    address_space.insert(node, Some(&[
        (&NodeId::new(0, 2253), ReferenceTypeId::Organizes, ReferenceDirection::Inverse),
        (&NodeId::new(0, 12750), ReferenceTypeId::HasProperty, ReferenceDirection::Forward),
        (&NodeId::new(0, 12751), ReferenceTypeId::HasProperty, ReferenceDirection::Forward),
        (&NodeId::new(0, 2253), ReferenceTypeId::HasComponent, ReferenceDirection::Inverse),
    ]));
}

fn add_method_12(address_space: &mut AddressSpace) {
    // Method
    let browse_name = "RequestServerStateChange";
    let display_name = "RequestServerStateChange";
    let description = "";
    let node_id = NodeId::new(0, 12886);
    let node = Method::new(&node_id, browse_name, display_name, description, false, false, false);
    address_space.insert(node, Some(&[
        (&NodeId::new(0, 2253), ReferenceTypeId::Organizes, ReferenceDirection::Inverse),
        (&NodeId::new(0, 12887), ReferenceTypeId::HasProperty, ReferenceDirection::Forward),
        (&NodeId::new(0, 2253), ReferenceTypeId::HasComponent, ReferenceDirection::Inverse),
    ]));
}

