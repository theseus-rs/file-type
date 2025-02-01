use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27967208: FileFormat = FileFormat {
    id: 27_967_208,
    puid: "wikidata/27967208",
    name: "Pro Tracker v2.xx module",
    extensions: &["pt2"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
