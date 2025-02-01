use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_979630: FileFormat = FileFormat {
    id: 979_630,
    puid: "wikidata/979630",
    name: "Industry Foundation Classes",
    extensions: &[
        "ifc", "ifc", "ifc", "ifc", "ifcXML", "ifcXML", "ifcXML", "ifcXML", "ifczip", "ifczip",
        "ifczip", "ifczip",
    ],
    media_types: &[
        "application/octet-stream",
        "application/octet-stream",
        "application/octet-stream",
        "application/octet-stream",
        "application/octet-stream",
        "application/octet-stream",
        "application/octet-stream",
        "application/octet-stream",
        "application/octet-stream",
        "application/octet-stream",
        "application/octet-stream",
        "application/octet-stream",
    ],
    internal_signatures: &[],
    related_formats: &[],
};
