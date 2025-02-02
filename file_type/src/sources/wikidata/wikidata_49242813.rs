use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_49242813: FileFormat = FileFormat {
    id: 49_242_813,
    source_type: SourceType::Wikidata,
    name: "HTML Extension File",
    extensions: &["htx"],
    media_types: &["text/html"],
    internal_signatures: &[],
    related_formats: &[],
};
