use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_25313036: FileFormat = FileFormat {
    id: 25_313_036,
    puid: "wikidata/25313036",
    name: "Extensible Data Notation",
    extensions: &["edn"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
