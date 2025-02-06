use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_117663636: FileFormat = FileFormat {
    id: 117_663_636,
    source_type: SourceType::Wikidata,
    name: "The Print Shop Quick Prints File",
    extensions: &["php"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
