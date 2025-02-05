use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_109997009: FileFormat = FileFormat {
    id: 109_997_009,
    source_type: SourceType::Wikidata,
    name: "OrgPlus 4 Template",
    extensions: &["ops"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
