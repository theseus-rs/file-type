use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_131278668: FileFormat = FileFormat {
    id: 131_278_668,
    source_type: SourceType::Wikidata,
    name: "Turbo Assembler assembly code file",
    extensions: &["asm", "tasm"],
    media_types: &["text/x-tasm"],
    signatures: &[],
    related_formats: &[],
};
