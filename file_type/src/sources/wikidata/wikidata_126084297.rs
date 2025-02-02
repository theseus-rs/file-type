use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_126084297: FileFormat = FileFormat {
    id: 126_084_297,
    source_type: SourceType::Wikidata,
    name: "SPIR-V file",
    extensions: &["spirv"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
