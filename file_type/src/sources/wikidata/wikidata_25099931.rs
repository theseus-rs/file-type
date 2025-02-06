use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_25099931: FileFormat = FileFormat {
    id: 25_099_931,
    source_type: SourceType::Wikidata,
    name: "Scratch Project SB2",
    extensions: &["sb2"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
