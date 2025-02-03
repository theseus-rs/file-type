use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28757918: FileFormat = FileFormat {
    id: 28_757_918,
    source_type: SourceType::Wikidata,
    name: "Google Sheet",
    extensions: &["gsheet"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
