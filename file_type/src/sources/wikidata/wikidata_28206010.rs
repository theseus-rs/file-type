use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28206010: FileFormat = FileFormat {
    id: 28_206_010,
    source_type: SourceType::Wikidata,
    name: "Digital Video Interactive Y Luminance Channel (Compressed 8-bit)",
    extensions: &["cmy"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
