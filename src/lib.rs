//
//

mod transform;
mod api;
pub mod settings;

pub const PUB28_STANDARDS_DESC: [(&str, &str); 24] = [
    ("212", "All uppercase letters"),
    ("213", "Use official abbreviations for sceondary address designators"),
    ("213.1", "All common designators use official abbreviations (see `pub28_c2.csv`)"),
    ("213.2", "Space between pound (#) sign and secondary number"),
    ("222", "Remove punctuation"),
    ("223", "Replace N/S/E/W with NORTH/SOUTH/EAST/WEST in city names"),
    ("232", "Remove hyphens from street and city names"),
    ("233.1", "Use official one- or two-letter abbreviation for the 8 directionals"),
    ("233.23", "If two directionals appear side by side in a street, spell them out fully (eg SOUTHWEST or NORTH EAST)"),
    ("233.3", "If a directional appears between the street name and suffix, spell it out (eg BAY WEST DR, NORTH AVE)"),
    ("234.1", "Use official abbreviations for suffixes"),
    ("234.2", "If two suffixes appear side by side, abbreviate the last one but spell out the first one"),
    ("241", "Abbreviate rural route addresses to RR ## BOX ##"),
    ("242", "Remove leading zero from RR number"),
    ("244", "Change RFD (rural-free delivery) and RD (rural delivery) designations to RR"),
    ("245", "Remove all other address designations form a rural route address (other than RR ## BOX ##)"),
    ("251", "Abbreviate highway contract route addresses to HC ## BOX ##"),
    ("252", "Remove leading zero from HC number"),
    ("253", "Abbreviate STAR ROUTE as HC"),
    ("254", "Remove all other address designations from a highway contract route address (other than HC ## BOX ##)"),
    ("281", "Abbreviate PO BOX addresses to PO BOX ###"),
    ("282", "If a PO BOX number starts with a hyphen, replace it with a zero"),
    ("283", "Replace CALLER, FIRM CALLER, BIN, LOCKBOX, or DRAWER with PO BOX"),
    ("284", "If a pound (#) sign appears after a street suffix followed by a number, replace the pound sign with UNIT"),
];
