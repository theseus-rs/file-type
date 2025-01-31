use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_113486673: FileFormat = FileFormat {
    id: 113_486_673,
    puid: "wikidata/113486673",
    name: "Persuasion Mac Document 2.1",
    extensions: &["pr2"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
