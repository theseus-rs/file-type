use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111009231: FileFormat = FileFormat {
    id: 111_009_231,
    puid: "wikidata/111009231",
    name: "PrintMaster Poster File format",
    extensions: &["sig"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
