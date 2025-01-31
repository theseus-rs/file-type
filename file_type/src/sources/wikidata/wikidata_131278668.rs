use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_131278668: FileFormat = FileFormat {
    id: 131_278_668,
    puid: "wikidata/131278668",
    name: "Turbo Assembler assembly code file",
    extensions: &["asm", "tasm"],
    media_types: &["text/x-tasm", "text/x-tasm"],
    internal_signatures: &[],
    related_formats: &[],
};
