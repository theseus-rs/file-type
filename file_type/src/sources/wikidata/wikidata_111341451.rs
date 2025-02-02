use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111341451: FileFormat = FileFormat {
    id: 111_341_451,
    source_type: SourceType::Wikidata,
    name: "ScreamTracker v3 sample",
    extensions: &["s3i"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
