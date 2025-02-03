use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_59821004: FileFormat = FileFormat {
    id: 59_821_004,
    source_type: SourceType::Wikidata,
    name: "Exchangeable Image File Format (Audio)",
    extensions: &["wav"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
