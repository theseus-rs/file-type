use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_372626: FileFormat = FileFormat {
    id: 372_626,
    puid: "wikidata/372626",
    name: "Theora",
    extensions: &["ogg", "ogv"],
    media_types: &["video/theora", "video/theora"],
    internal_signatures: &[],
    related_formats: &[],
};
