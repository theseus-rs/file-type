use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_113085760: FileFormat = FileFormat {
    id: 113_085_760,
    source_type: SourceType::Wikidata,
    name: "CB7",
    extensions: &["cb7"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
