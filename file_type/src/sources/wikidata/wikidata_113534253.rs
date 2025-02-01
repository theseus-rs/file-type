use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_113534253: FileFormat = FileFormat {
    id: 113_534_253,
    puid: "wikidata/113534253",
    name: "Geosoft Map Description File",
    extensions: &["mdf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
