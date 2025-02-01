use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_29904453: FileFormat = FileFormat {
    id: 29_904_453,
    puid: "wikidata/29904453",
    name: "PowerPacker",
    extensions: &["pp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
