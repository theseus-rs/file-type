use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27824015: FileFormat = FileFormat {
    id: 27_824_015,
    puid: "wikidata/27824015",
    name: "ar, BSD variant",
    extensions: &["a"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
