use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_3551300: FileFormat = FileFormat {
    id: 3_551_300,
    puid: "wikidata/3551300",
    name: "Universal Subtitle Format",
    extensions: &["xml"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
