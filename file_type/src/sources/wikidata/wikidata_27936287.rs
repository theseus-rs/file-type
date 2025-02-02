use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27936287: FileFormat = FileFormat {
    id: 27_936_287,
    source_type: SourceType::Wikidata,
    name: "Earth Resources Laboratory Applications Software",
    extensions: &["elas"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
