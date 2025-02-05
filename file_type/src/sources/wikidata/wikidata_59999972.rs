use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_59999972: FileFormat = FileFormat {
    id: 59_999_972,
    source_type: SourceType::Wikidata,
    name: "Borland Reflex flat datafile",
    extensions: &["rxd"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
