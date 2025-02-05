use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_52063281: FileFormat = FileFormat {
    id: 52_063_281,
    source_type: SourceType::Wikidata,
    name: "SAS Data File",
    extensions: &["ssd"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
