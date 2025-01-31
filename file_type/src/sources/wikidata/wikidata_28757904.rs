use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28757904: FileFormat = FileFormat {
    id: 28_757_904,
    puid: "wikidata/28757904",
    name: "Go script",
    extensions: &["go"],
    media_types: &["text/x-gosrc"],
    internal_signatures: &[],
    related_formats: &[],
};
