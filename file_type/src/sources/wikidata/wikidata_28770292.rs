use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28770292: FileFormat = FileFormat {
    id: 28_770_292,
    source_type: SourceType::Wikidata,
    name: "LZ4",
    extensions: &["lz4"],
    media_types: &["application/x-lz4"],
    signatures: &[],
    related_formats: &[],
};
