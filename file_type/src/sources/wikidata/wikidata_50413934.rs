use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_50413934: FileFormat = FileFormat {
    id: 50_413_934,
    source_type: SourceType::Wikidata,
    name: "Lightwright 1 Show File",
    extensions: &["lw1"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[],
    related_formats: &[],
};
