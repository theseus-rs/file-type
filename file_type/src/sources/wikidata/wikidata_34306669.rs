use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_34306669: FileFormat = FileFormat {
    id: 34_306_669,
    puid: "wikidata/34306669",
    name: "Scifer archive XML header",
    extensions: &["xml"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
