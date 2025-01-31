use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105852844: FileFormat = FileFormat {
    id: 105_852_844,
    puid: "wikidata/105852844",
    name: "SuperTux World Map File (with rem)",
    extensions: &["stwm"],
    media_types: &["text/plain"],
    internal_signatures: &[],
    related_formats: &[],
};
