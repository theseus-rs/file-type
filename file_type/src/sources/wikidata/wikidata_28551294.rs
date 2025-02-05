use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28551294: FileFormat = FileFormat {
    id: 28_551_294,
    source_type: SourceType::Wikidata,
    name: "Adobe Color Table File",
    extensions: &["act"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
