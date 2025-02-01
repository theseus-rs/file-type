use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_113486423: FileFormat = FileFormat {
    id: 113_486_423,
    puid: "wikidata/113486423",
    name: "Persuasion Mac Document 1.0",
    extensions: &["pr1"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
