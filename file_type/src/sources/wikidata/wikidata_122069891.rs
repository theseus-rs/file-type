use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_122069891: FileFormat = FileFormat {
    id: 122_069_891,
    puid: "wikidata/122069891",
    name: "Post-It Software Note File",
    extensions: &["ppn"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
