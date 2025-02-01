use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_123296707: FileFormat = FileFormat {
    id: 123_296_707,
    puid: "wikidata/123296707",
    name: "CD-Face Layout",
    extensions: &["ntp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
