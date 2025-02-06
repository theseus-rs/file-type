use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28757992: FileFormat = FileFormat {
    id: 28_757_992,
    source_type: SourceType::Wikidata,
    name: "ISZ",
    extensions: &["isz"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
