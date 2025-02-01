use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_49619991: FileFormat = FileFormat {
    id: 49_619_991,
    puid: "wikidata/49619991",
    name: "Revit External Group",
    extensions: &["rvg"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[],
    related_formats: &[],
};
