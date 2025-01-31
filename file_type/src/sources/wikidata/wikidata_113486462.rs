use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_113486462: FileFormat = FileFormat {
    id: 113_486_462,
    puid: "wikidata/113486462",
    name: "Persuasion Mac Document 2.0",
    extensions: &["pr2"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
