use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_44933672: FileFormat = FileFormat {
    id: 44_933_672,
    source_type: SourceType::Wikidata,
    name: "dBASE IV file format",
    extensions: &["dbf"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
