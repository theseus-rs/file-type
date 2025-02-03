use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_113674320: FileFormat = FileFormat {
    id: 113_674_320,
    source_type: SourceType::Wikidata,
    name: "Final Draft 8 Template",
    extensions: &["fdxt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
