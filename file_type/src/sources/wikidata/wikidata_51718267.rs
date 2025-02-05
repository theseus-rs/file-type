use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_51718267: FileFormat = FileFormat {
    id: 51_718_267,
    source_type: SourceType::Wikidata,
    name: "Schedule+ Contacts",
    extensions: &["scd"],
    media_types: &["application/octet-stream"],
    signatures: &[],
    related_formats: &[],
};
