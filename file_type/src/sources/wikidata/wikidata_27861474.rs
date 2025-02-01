use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27861474: FileFormat = FileFormat {
    id: 27_861_474,
    puid: "wikidata/27861474",
    name: "Software Independent Archiving of Relational Databases, version 2.0",
    extensions: &["siard"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
