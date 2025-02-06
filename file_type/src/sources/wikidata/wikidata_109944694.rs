use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_109944694: FileFormat = FileFormat {
    id: 109_944_694,
    source_type: SourceType::Wikidata,
    name: "Archives file format",
    extensions: &["arv"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
