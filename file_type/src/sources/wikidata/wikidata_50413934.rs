use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_50413934: FileFormat = FileFormat {
    id: 50_413_934,
    source_type: SourceType::Wikidata,
    name: "Lightwright 1 Show File",
    extensions: &["lw1"],
    media_types: &["application/octet-stream"],
    signatures: &[],
    related_formats: &[],
};
