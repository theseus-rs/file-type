use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28205801: FileFormat = FileFormat {
    id: 28_205_801,
    source_type: SourceType::Wikidata,
    name: "IMG Picture Format",
    extensions: &["img"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
