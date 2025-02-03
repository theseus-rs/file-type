use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28052851: FileFormat = FileFormat {
    id: 28_052_851,
    source_type: SourceType::Wikidata,
    name: "RePub",
    extensions: &["epub"],
    media_types: &["application/epub+zip"],
    internal_signatures: &[],
    related_formats: &[],
};
