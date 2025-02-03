use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105860739: FileFormat = FileFormat {
    id: 105_860_739,
    source_type: SourceType::Wikidata,
    name: "RemObjects Definition Language",
    extensions: &["rodl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
