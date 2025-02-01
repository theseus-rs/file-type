use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_29904619: FileFormat = FileFormat {
    id: 29_904_619,
    puid: "wikidata/29904619",
    name: "Statistical Analysis System access descriptor file",
    extensions: &["sa2", "sa7", "sas7bacs"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
