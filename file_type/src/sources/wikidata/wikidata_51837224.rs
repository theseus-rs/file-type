use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_51837224: FileFormat = FileFormat {
    id: 51_837_224,
    source_type: SourceType::Wikidata,
    name: "Paradox Database Table, version 7",
    extensions: &["db"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
