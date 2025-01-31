use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_122411453: FileFormat = FileFormat {
    id: 122_411_453,
    puid: "wikidata/122411453",
    name: "Palm Pilot Symbol File",
    extensions: &["psym"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
