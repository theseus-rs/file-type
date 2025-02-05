use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_127120962: FileFormat = FileFormat {
    id: 127_120_962,
    source_type: SourceType::Wikidata,
    name: "Matrix Market file format",
    extensions: &["mm"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
