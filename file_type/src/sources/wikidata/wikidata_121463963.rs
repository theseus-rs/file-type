use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_121463963: FileFormat = FileFormat {
    id: 121_463_963,
    source_type: SourceType::Wikidata,
    name: "Adobe Lightroom Library",
    extensions: &["aglib"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
