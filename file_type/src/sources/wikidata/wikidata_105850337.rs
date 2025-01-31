use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105850337: FileFormat = FileFormat {
    id: 105_850_337,
    puid: "wikidata/105850337",
    name: "Cabbage script (with rem)",
    extensions: &["csd"],
    media_types: &["text/plain"],
    internal_signatures: &[],
    related_formats: &[],
};
