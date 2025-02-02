use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_125692127: FileFormat = FileFormat {
    id: 125_692_127,
    source_type: SourceType::Wikidata,
    name: "OpenDocument Presentation Template",
    extensions: &["otp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
