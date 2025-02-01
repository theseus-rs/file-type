use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_29904545: FileFormat = FileFormat {
    id: 29_904_545,
    puid: "wikidata/29904545",
    name: "Statistical Analysis System data set index",
    extensions: &["sas7bndx", "si2", "si7"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
