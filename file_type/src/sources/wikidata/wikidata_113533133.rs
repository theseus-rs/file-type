use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_113533133: FileFormat = FileFormat {
    id: 113_533_133,
    puid: "wikidata/113533133",
    name: "LegalDocML Document",
    extensions: &["xml"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
