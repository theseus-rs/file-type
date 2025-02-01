use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_29904540: FileFormat = FileFormat {
    id: 29_904_540,
    puid: "wikidata/29904540",
    name: "Statistical Analysis System log file",
    extensions: &["log"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
