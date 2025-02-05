use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_130386942: FileFormat = FileFormat {
    id: 130_386_942,
    source_type: SourceType::Wikidata,
    name: "objdump file format",
    extensions: &["objdump"],
    media_types: &["text/x-objdump"],
    signatures: &[],
    related_formats: &[],
};
