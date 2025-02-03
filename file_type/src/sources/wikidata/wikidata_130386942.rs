use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_130386942: FileFormat = FileFormat {
    id: 130_386_942,
    source_type: SourceType::Wikidata,
    name: "objdump file format",
    extensions: &["objdump"],
    media_types: &["text/x-objdump"],
    internal_signatures: &[],
    related_formats: &[],
};
