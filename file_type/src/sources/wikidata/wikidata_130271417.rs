use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_130271417: FileFormat = FileFormat {
    id: 130_271_417,
    source_type: SourceType::Wikidata,
    name: "Mako file format",
    extensions: &["mao"],
    media_types: &["application/x-mako"],
    signatures: &[],
    related_formats: &[],
};
