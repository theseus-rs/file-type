use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_29905141: FileFormat = FileFormat {
    id: 29_905_141,
    puid: "wikidata/29905141",
    name: "Statistical Analysis System utility file",
    extensions: &["sas7butl", "su2", "su7"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
