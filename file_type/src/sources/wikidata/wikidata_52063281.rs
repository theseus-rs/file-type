use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_52063281: FileFormat = FileFormat {
    id: 52_063_281,
    source_type: SourceType::Wikidata,
    name: "SAS Data File",
    extensions: &["ssd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
