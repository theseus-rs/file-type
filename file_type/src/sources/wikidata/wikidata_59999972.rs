use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_59999972: FileFormat = FileFormat {
    id: 59_999_972,
    source_type: SourceType::Wikidata,
    name: "Borland Reflex flat datafile",
    extensions: &["rxd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
