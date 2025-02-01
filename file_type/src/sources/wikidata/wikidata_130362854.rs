use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_130362854: FileFormat = FileFormat {
    id: 130_362_854,
    puid: "wikidata/130362854",
    name: "Netwide Assembler file format",
    extensions: &["asm", "nasm"],
    media_types: &["text/x-nasm", "text/x-nasm"],
    internal_signatures: &[],
    related_formats: &[],
};
