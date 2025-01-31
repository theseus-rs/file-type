use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28345358: FileFormat = FileFormat {
    id: 28_345_358,
    puid: "wikidata/28345358",
    name: "Safari bookmarks",
    extensions: &["plist"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
