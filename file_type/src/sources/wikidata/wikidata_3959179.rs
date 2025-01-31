use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_3959179: FileFormat = FileFormat {
    id: 3_959_179,
    puid: "wikidata/3959179",
    name: "shar",
    extensions: &["sha", "shar"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
