use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_29651120: FileFormat = FileFormat {
    id: 29_651_120,
    source_type: SourceType::Wikidata,
    name: "Personal Folder File",
    extensions: &["pst"],
    media_types: &["application/vnd.ms-outlook"],
    internal_signatures: &[],
    related_formats: &[],
};
