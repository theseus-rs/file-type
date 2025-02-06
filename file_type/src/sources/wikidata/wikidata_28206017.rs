use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28206017: FileFormat = FileFormat {
    id: 28_206_017,
    source_type: SourceType::Wikidata,
    name: "Digital Video Interactive Q Color Channel (Compressed 8-bit)",
    extensions: &["cmq"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
