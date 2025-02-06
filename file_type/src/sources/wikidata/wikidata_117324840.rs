use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_117324840: FileFormat = FileFormat {
    id: 117_324_840,
    source_type: SourceType::Wikidata,
    name: "Function Tree file",
    extensions: &["fp"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
