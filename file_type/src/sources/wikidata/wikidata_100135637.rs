use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_100135637: FileFormat = FileFormat {
    id: 100_135_637,
    source_type: SourceType::Wikidata,
    name: "XDOMEA 2.1.0",
    extensions: &["xml"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
