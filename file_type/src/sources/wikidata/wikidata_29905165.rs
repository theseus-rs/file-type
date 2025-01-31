use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_29905165: FileFormat = FileFormat {
    id: 29_905_165,
    puid: "wikidata/29905165",
    name: "Statistical Analysis System backup file",
    extensions: &["sas7bbak", "sb7"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
