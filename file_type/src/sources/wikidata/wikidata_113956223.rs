use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_113956223: FileFormat = FileFormat {
    id: 113_956_223,
    puid: "wikidata/113956223",
    name: "Software-Independent Archiving of Relational Databases, version 2.1",
    extensions: &["siard"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
