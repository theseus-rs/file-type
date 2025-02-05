use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28206866: FileFormat = FileFormat {
    id: 28_206_866,
    source_type: SourceType::Wikidata,
    name: "PCPaint clipping format",
    extensions: &["clp"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
