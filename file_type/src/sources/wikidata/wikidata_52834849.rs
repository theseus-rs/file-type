use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_52834849: FileFormat = FileFormat {
    id: 52_834_849,
    puid: "wikidata/52834849",
    name: "dBASE for Windows database, version 5",
    extensions: &["dbf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
