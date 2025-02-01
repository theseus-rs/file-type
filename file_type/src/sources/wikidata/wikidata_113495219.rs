use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_113495219: FileFormat = FileFormat {
    id: 113_495_219,
    puid: "wikidata/113495219",
    name: "CATIA Model File 3",
    extensions: &["model"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
