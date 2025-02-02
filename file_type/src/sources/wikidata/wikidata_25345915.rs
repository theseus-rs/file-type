use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_25345915: FileFormat = FileFormat {
    id: 25_345_915,
    source_type: SourceType::Wikidata,
    name: "Scratch Project SB",
    extensions: &["sb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
