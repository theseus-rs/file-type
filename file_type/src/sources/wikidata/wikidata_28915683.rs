use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28915683: FileFormat = FileFormat {
    id: 28_915_683,
    puid: "wikidata/28915683",
    name: "Apache Parquet",
    extensions: &["parquet"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
