use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_113486947: FileFormat = FileFormat {
    id: 113_486_947,
    puid: "wikidata/113486947",
    name: "Persuasion Mac Document 3.0",
    extensions: &["pr3"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
