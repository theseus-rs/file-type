use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_85415600: FileFormat = FileFormat {
    id: 85_415_600,
    source_type: SourceType::Wikidata,
    name: "Tweet JSON",
    extensions: &["json"],
    media_types: &["application/json"],
    signatures: &[],
    related_formats: &[],
};
