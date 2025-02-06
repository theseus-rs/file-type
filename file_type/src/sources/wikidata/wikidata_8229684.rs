use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_8229684: FileFormat = FileFormat {
    id: 8_229_684,
    source_type: SourceType::Wikidata,
    name: "Elf",
    extensions: &["elf"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
