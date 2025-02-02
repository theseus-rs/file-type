use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_130386156: FileFormat = FileFormat {
    id: 130_386_156,
    source_type: SourceType::Wikidata,
    name: "Nit source code file",
    extensions: &["nit"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
