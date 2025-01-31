use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28757836: FileFormat = FileFormat {
    id: 28_757_836,
    puid: "wikidata/28757836",
    name: "Geany project",
    extensions: &["geany"],
    media_types: &["text/ini"],
    internal_signatures: &[],
    related_formats: &[],
};
