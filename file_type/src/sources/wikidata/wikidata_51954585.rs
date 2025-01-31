use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_51954585: FileFormat = FileFormat {
    id: 51_954_585,
    puid: "wikidata/51954585",
    name: "dBASE III+ file format",
    extensions: &["dbf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
