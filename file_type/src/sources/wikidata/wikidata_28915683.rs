use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28915683: FileFormat = FileFormat {
    id: 28_915_683,
    source_type: SourceType::Wikidata,
    name: "Apache Parquet",
    extensions: &["parquet"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
