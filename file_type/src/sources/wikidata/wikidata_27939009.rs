use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27939009: FileFormat = FileFormat {
    id: 27_939_009,
    source_type: SourceType::Wikidata,
    name: "Enhanced Compression Wavelet, version 2",
    extensions: &["ecw"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
