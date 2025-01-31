use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28600255: FileFormat = FileFormat {
    id: 28_600_255,
    puid: "wikidata/28600255",
    name: "ARTS",
    extensions: &["arts"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
