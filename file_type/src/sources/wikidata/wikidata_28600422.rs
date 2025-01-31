use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28600422: FileFormat = FileFormat {
    id: 28_600_422,
    puid: "wikidata/28600422",
    name: "4bottle",
    extensions: &["4q"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
