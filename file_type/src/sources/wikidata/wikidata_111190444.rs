use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111190444: FileFormat = FileFormat {
    id: 111_190_444,
    puid: "wikidata/111190444",
    name: "Java Script Command File",
    extensions: &["jsf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
