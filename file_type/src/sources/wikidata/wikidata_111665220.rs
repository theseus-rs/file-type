use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111665220: FileFormat = FileFormat {
    id: 111_665_220,
    source_type: SourceType::Wikidata,
    name: "OpenDocument Text Template",
    extensions: &["ott"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
