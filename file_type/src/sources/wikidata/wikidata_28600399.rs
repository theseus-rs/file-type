use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28600399: FileFormat = FileFormat {
    id: 28_600_399,
    puid: "wikidata/28600399",
    name: "Arma PBO",
    extensions: &["pbo"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
