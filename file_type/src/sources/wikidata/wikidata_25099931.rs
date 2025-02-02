use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_25099931: FileFormat = FileFormat {
    id: 25_099_931,
    source_type: SourceType::Wikidata,
    name: "Scratch Project SB2",
    extensions: &["sb2"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
