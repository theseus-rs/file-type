use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_29651120: FileFormat = FileFormat {
    id: 29_651_120,
    puid: "wikidata/29651120",
    name: "Personal Folder File",
    extensions: &["pst"],
    media_types: &["application/vnd.ms-outlook"],
    internal_signatures: &[],
    related_formats: &[],
};
