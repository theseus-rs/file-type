use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_29904541: FileFormat = FileFormat {
    id: 29_904_541,
    puid: "wikidata/29904541",
    name: "Statistical Analysis System audit file",
    extensions: &["sas7baud", "st7"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
