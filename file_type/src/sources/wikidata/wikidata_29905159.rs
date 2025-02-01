use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_29905159: FileFormat = FileFormat {
    id: 29_905_159,
    puid: "wikidata/29905159",
    name: "Statistical Analysis System transport file",
    extensions: &["stx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
