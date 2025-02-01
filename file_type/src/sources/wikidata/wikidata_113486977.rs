use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_113486977: FileFormat = FileFormat {
    id: 113_486_977,
    puid: "wikidata/113486977",
    name: "Persuasion Mac Document 4.0",
    extensions: &["pn4"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
