use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_125148800: FileFormat = FileFormat {
    id: 125_148_800,
    puid: "wikidata/125148800",
    name: "YAM Users",
    extensions: &["users"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
