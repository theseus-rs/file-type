use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28207188: FileFormat = FileFormat {
    id: 28_207_188,
    source_type: SourceType::Wikidata,
    name: "QDV",
    extensions: &["qdv"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
