use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28052851: FileFormat = FileFormat {
    id: 28_052_851,
    source_type: SourceType::Wikidata,
    name: "RePub",
    extensions: &["epub"],
    media_types: &["application/epub+zip"],
    signatures: &[],
    related_formats: &[],
};
