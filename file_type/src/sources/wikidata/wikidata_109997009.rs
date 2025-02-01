use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_109997009: FileFormat = FileFormat {
    id: 109_997_009,
    puid: "wikidata/109997009",
    name: "OrgPlus 4 Template",
    extensions: &["ops"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
