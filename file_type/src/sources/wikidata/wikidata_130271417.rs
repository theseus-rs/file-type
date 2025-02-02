use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_130271417: FileFormat = FileFormat {
    id: 130_271_417,
    source_type: SourceType::Wikidata,
    name: "Mako file format",
    extensions: &["mao"],
    media_types: &["application/x-mako"],
    internal_signatures: &[],
    related_formats: &[],
};
