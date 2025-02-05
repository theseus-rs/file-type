use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27936287: FileFormat = FileFormat {
    id: 27_936_287,
    source_type: SourceType::Wikidata,
    name: "Earth Resources Laboratory Applications Software",
    extensions: &["elas"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
