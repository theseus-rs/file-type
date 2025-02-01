use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27967209: FileFormat = FileFormat {
    id: 27_967_209,
    puid: "wikidata/27967209",
    name: "Pro Tracker v3.xx module",
    extensions: &["pt3"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
