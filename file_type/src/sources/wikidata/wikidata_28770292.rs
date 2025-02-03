use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28770292: FileFormat = FileFormat {
    id: 28_770_292,
    source_type: SourceType::Wikidata,
    name: "LZ4",
    extensions: &["lz4"],
    media_types: &["application/x-lz4"],
    internal_signatures: &[],
    related_formats: &[],
};
