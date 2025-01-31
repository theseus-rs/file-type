use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111721962: FileFormat = FileFormat {
    id: 111_721_962,
    puid: "wikidata/111721962",
    name: "Fortran include file",
    extensions: &["i90", "inc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
