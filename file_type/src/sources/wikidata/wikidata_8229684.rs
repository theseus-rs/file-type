use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_8229684: FileFormat = FileFormat {
    id: 8_229_684,
    puid: "wikidata/8229684",
    name: "Elf",
    extensions: &["elf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
