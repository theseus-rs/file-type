use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_124097900: FileFormat = FileFormat {
    id: 124_097_900,
    source_type: SourceType::Wikidata,
    name: ".txt file",
    extensions: &["txt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
