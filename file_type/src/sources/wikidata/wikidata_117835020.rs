use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_117835020: FileFormat = FileFormat {
    id: 117_835_020,
    puid: "wikidata/117835020",
    name: "Canon Navigator Fax file",
    extensions: &["can"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
