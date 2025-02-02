use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105851182: FileFormat = FileFormat {
    id: 105_851_182,
    source_type: SourceType::Wikidata,
    name: "LaTeX table of contents",
    extensions: &["toc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
