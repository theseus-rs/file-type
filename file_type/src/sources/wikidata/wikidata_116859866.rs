use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_116859866: FileFormat = FileFormat {
    id: 116_859_866,
    source_type: SourceType::Wikidata,
    name: "Lesson File",
    extensions: &["lsn"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
