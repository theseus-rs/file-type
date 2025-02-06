use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_130362854: FileFormat = FileFormat {
    id: 130_362_854,
    source_type: SourceType::Wikidata,
    name: "Netwide Assembler file format",
    extensions: &["asm", "nasm"],
    media_types: &["text/x-nasm"],
    signatures: &[],
    related_formats: &[],
};
