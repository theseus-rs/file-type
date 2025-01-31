use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_117834929: FileFormat = FileFormat {
    id: 117_834_929,
    puid: "wikidata/117834929",
    name: "AT&T Group 4 file",
    extensions: &["att"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
