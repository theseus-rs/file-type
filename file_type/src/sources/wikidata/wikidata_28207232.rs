use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28207232: FileFormat = FileFormat {
    id: 28_207_232,
    source_type: SourceType::Wikidata,
    name: "RLA",
    extensions: &["rla"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
