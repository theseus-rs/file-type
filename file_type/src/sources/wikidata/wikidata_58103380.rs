use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_58103380: FileFormat = FileFormat {
    id: 58_103_380,
    puid: "wikidata/58103380",
    name: "eRuby HTML document",
    extensions: &["rhtm", "rhtml"],
    media_types: &["text/html+ruby", "text/html+ruby"],
    internal_signatures: &[],
    related_formats: &[],
};
