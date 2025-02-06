use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_113162672: FileFormat = FileFormat {
    id: 113_162_672,
    source_type: SourceType::Wikidata,
    name: "MyContactManager File",
    extensions: &["mcm"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
