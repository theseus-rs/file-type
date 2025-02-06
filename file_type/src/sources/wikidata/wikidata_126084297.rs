use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_126084297: FileFormat = FileFormat {
    id: 126_084_297,
    source_type: SourceType::Wikidata,
    name: "SPIR-V file",
    extensions: &["spirv"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
