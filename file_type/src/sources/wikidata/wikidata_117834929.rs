use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_117834929: FileFormat = FileFormat {
    id: 117_834_929,
    source_type: SourceType::Wikidata,
    name: "AT&T Group 4 file",
    extensions: &["att"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
