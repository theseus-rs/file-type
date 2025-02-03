use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_114891709: FileFormat = FileFormat {
    id: 114_891_709,
    source_type: SourceType::Wikidata,
    name: "Tax Export File",
    extensions: &["txf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
