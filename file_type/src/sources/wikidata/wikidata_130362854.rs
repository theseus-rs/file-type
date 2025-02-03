use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_130362854: FileFormat = FileFormat {
    id: 130_362_854,
    source_type: SourceType::Wikidata,
    name: "Netwide Assembler file format",
    extensions: &["asm", "nasm"],
    media_types: &["text/x-nasm"],
    internal_signatures: &[],
    related_formats: &[],
};
