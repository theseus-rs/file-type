use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_10376670: FileFormat = FileFormat {
    id: 10_376_670,
    source_type: SourceType::Wikidata,
    name: "tar.bz2",
    extensions: &["tar.bz2"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
