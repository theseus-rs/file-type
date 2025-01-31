use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_123359482: FileFormat = FileFormat {
    id: 123_359_482,
    puid: "wikidata/123359482",
    name: "Personal History Project",
    extensions: &["phst"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
