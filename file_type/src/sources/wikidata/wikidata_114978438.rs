use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_114978438: FileFormat = FileFormat {
    id: 114_978_438,
    puid: "wikidata/114978438",
    name: "StoryView Document",
    extensions: &["syv"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
