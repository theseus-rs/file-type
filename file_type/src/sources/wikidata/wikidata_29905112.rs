use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_29905112: FileFormat = FileFormat {
    id: 29_905_112,
    puid: "wikidata/29905112",
    name: "Statistical Analysis System item store file",
    extensions: &["sas7bitm", "sr7"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
