use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_50288226: FileFormat = FileFormat {
    id: 50_288_226,
    source_type: SourceType::Wikidata,
    name: "Adobe Air, v1.5",
    extensions: &["air"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
