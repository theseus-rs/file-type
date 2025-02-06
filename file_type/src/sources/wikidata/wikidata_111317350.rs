use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111317350: FileFormat = FileFormat {
    id: 111_317_350,
    source_type: SourceType::Wikidata,
    name: "Matlab variable binary file",
    extensions: &["mat"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
