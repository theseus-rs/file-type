use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_79242927: FileFormat = FileFormat {
    id: 79_242_927,
    source_type: SourceType::Wikidata,
    name: "Adobe After Effects Graphics",
    extensions: &["aegraphic"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
