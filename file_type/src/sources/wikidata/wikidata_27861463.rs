use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27861463: FileFormat = FileFormat {
    id: 27_861_463,
    puid: "wikidata/27861463",
    name: "Software Independent Archiving of Relational Databases, version 1.0",
    extensions: &["siard"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
