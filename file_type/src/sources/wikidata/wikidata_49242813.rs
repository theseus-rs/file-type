use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_49242813: FileFormat = FileFormat {
    id: 49_242_813,
    source_type: SourceType::Wikidata,
    name: "HTML Extension File",
    extensions: &["htx"],
    media_types: &["text/html"],
    signatures: &[],
    related_formats: &[],
};
