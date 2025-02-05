use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_130386156: FileFormat = FileFormat {
    id: 130_386_156,
    source_type: SourceType::Wikidata,
    name: "Nit source code file",
    extensions: &["nit"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
