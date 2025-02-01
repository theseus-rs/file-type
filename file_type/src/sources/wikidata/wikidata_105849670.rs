use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105849670: FileFormat = FileFormat {
    id: 105_849_670,
    puid: "wikidata/105849670",
    name: "ArtCAM post processor Configuration (with rem)",
    extensions: &["con"],
    media_types: &["text/plain"],
    internal_signatures: &[],
    related_formats: &[],
};
