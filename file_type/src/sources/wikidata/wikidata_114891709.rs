use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_114891709: FileFormat = FileFormat {
    id: 114_891_709,
    source_type: SourceType::Wikidata,
    name: "Tax Export File",
    extensions: &["txf"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
