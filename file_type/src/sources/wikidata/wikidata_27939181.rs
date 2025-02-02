use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27939181: FileFormat = FileFormat {
    id: 27_939_181,
    source_type: SourceType::Wikidata,
    name: "Enhanced Compression Wavelet, version 3",
    extensions: &["ecw"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
