use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_29651120: FileFormat = FileFormat {
    id: 29_651_120,
    source_type: SourceType::Wikidata,
    name: "Personal Folder File",
    extensions: &["pst"],
    media_types: &["application/vnd.ms-outlook"],
    signatures: &[],
    related_formats: &[],
};
