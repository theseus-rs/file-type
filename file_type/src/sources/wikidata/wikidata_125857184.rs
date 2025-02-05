use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_125857184: FileFormat = FileFormat {
    id: 125_857_184,
    source_type: SourceType::Wikidata,
    name: "C-- source code file",
    extensions: &["c--"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
