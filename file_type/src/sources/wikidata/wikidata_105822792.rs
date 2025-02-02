use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105822792: FileFormat = FileFormat {
    id: 105_822_792,
    source_type: SourceType::Wikidata,
    name: "AMDIS Target Compounds Library",
    extensions: &["MSL"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
