use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28206866: FileFormat = FileFormat {
    id: 28_206_866,
    source_type: SourceType::Wikidata,
    name: "PCPaint clipping format",
    extensions: &["clp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
