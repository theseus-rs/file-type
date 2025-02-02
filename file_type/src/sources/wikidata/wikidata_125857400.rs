use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_125857400: FileFormat = FileFormat {
    id: 125_857_400,
    source_type: SourceType::Wikidata,
    name: "JScript Encoded File",
    extensions: &["jse"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
