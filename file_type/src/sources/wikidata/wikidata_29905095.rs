use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_29905095: FileFormat = FileFormat {
    id: 29_905_095,
    puid: "wikidata/29905095",
    name: "Statistical Analysis System multi-dimensional database file",
    extensions: &["sas7bmdb", "sm2", "sm7"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
