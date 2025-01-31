use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_29650319: FileFormat = FileFormat {
    id: 29_650_319,
    puid: "wikidata/29650319",
    name: "PIM",
    extensions: &["pim"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
