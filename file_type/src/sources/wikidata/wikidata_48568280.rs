use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_48568280: FileFormat = FileFormat {
    id: 48_568_280,
    source_type: SourceType::Wikidata,
    name: "Lightwright 5 Show File",
    extensions: &["lw5"],
    media_types: &["application/octet-stream"],
    signatures: &[],
    related_formats: &[],
};
