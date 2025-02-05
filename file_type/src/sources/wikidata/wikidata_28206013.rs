use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28206013: FileFormat = FileFormat {
    id: 28_206_013,
    source_type: SourceType::Wikidata,
    name: "Digital Video Interactive I Color Channel (Compressed 8-bit)",
    extensions: &["cmi"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
