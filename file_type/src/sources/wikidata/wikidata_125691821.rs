use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_125691821: FileFormat = FileFormat {
    id: 125_691_821,
    puid: "wikidata/125691821",
    name: "OpenDocument Master Document",
    extensions: &["odm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
