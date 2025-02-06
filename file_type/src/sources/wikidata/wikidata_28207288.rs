use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28207288: FileFormat = FileFormat {
    id: 28_207_288,
    source_type: SourceType::Wikidata,
    name: "Slow-scan television",
    extensions: &["hrz"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
