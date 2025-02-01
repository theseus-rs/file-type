use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_681524: FileFormat = FileFormat {
    id: 681_524,
    puid: "wikidata/681524",
    name: "XML Shareable Playlist Format",
    extensions: &["xspf"],
    media_types: &["application/xspf+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
