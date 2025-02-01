use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_59821004: FileFormat = FileFormat {
    id: 59_821_004,
    puid: "wikidata/59821004",
    name: "Exchangeable Image File Format (Audio)",
    extensions: &["wav"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
