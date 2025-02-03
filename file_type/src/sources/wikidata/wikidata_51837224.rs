use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_51837224: FileFormat = FileFormat {
    id: 51_837_224,
    source_type: SourceType::Wikidata,
    name: "Paradox Database Table, version 7",
    extensions: &["db"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
