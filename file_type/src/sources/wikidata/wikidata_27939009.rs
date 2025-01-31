use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27939009: FileFormat = FileFormat {
    id: 27_939_009,
    puid: "wikidata/27939009",
    name: "Enhanced Compression Wavelet, version 2",
    extensions: &["ecw"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
