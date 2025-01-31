use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_129643497: FileFormat = FileFormat {
    id: 129_643_497,
    puid: "wikidata/129643497",
    name: "Icon file format",
    extensions: &["icon"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
