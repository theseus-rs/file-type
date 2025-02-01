use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27939181: FileFormat = FileFormat {
    id: 27_939_181,
    puid: "wikidata/27939181",
    name: "Enhanced Compression Wavelet, version 3",
    extensions: &["ecw"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
