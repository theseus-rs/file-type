use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_122069891: FileFormat = FileFormat {
    id: 122_069_891,
    source_type: SourceType::Wikidata,
    name: "Post-It Software Note File",
    extensions: &["ppn"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
