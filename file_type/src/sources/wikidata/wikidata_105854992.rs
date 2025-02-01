use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105854992: FileFormat = FileFormat {
    id: 105_854_992,
    puid: "wikidata/105854992",
    name: "ArcPad bookmarks",
    extensions: &["apx"],
    media_types: &["text/xml"],
    internal_signatures: &[],
    related_formats: &[],
};
