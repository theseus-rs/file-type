use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_120785583: FileFormat = FileFormat {
    id: 120_785_583,
    puid: "wikidata/120785583",
    name: "BusinessCards format",
    extensions: &["biz"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
