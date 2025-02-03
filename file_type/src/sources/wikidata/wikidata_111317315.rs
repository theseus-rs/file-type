use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111317315: FileFormat = FileFormat {
    id: 111_317_315,
    source_type: SourceType::Wikidata,
    name: "iPhone ring-tone",
    extensions: &["m4r"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
