use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_50413931: FileFormat = FileFormat {
    id: 50_413_931,
    source_type: SourceType::Wikidata,
    name: "Lightwright 2 Show File",
    extensions: &["lw2"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[],
    related_formats: &[],
};
