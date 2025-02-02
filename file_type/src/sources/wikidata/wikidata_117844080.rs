use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_117844080: FileFormat = FileFormat {
    id: 117_844_080,
    source_type: SourceType::Wikidata,
    name: "JetFax file",
    extensions: &["jet"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
