use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_116378812: FileFormat = FileFormat {
    id: 116_378_812,
    puid: "wikidata/116378812",
    name: "Act! Database File",
    extensions: &["dbf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
