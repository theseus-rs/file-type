use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105849670: FileFormat = FileFormat {
    id: 105_849_670,
    source_type: SourceType::Wikidata,
    name: "ArtCAM post processor Configuration (with rem)",
    extensions: &["con"],
    media_types: &["text/plain"],
    internal_signatures: &[],
    related_formats: &[],
};
