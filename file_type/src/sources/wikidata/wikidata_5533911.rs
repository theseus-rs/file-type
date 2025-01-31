use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_5533911: FileFormat = FileFormat {
    id: 5_533_911,
    puid: "wikidata/5533911",
    name: "GeoPDF",
    extensions: &["pdf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
