use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_47483863: FileFormat = FileFormat {
    id: 47_483_863,
    puid: "wikidata/47483863",
    name: "Statistical Analysis System Data (Unix)",
    extensions: &["sas7bdat", "sd7"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
