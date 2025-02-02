use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_131278668: FileFormat = FileFormat {
    id: 131_278_668,
    source_type: SourceType::Wikidata,
    name: "Turbo Assembler assembly code file",
    extensions: &["asm", "tasm"],
    media_types: &["text/x-tasm"],
    internal_signatures: &[],
    related_formats: &[],
};
