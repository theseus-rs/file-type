use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27967405: FileFormat = FileFormat {
    id: 27_967_405,
    puid: "wikidata/27967405",
    name: "Master Tracker module",
    extensions: &["mtr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
