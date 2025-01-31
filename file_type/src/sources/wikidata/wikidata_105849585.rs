use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105849585: FileFormat = FileFormat {
    id: 105_849_585,
    puid: "wikidata/105849585",
    name: "Construct 3 Project",
    extensions: &["c3p"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
