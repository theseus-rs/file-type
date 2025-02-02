use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_119443865: FileFormat = FileFormat {
    id: 119_443_865,
    source_type: SourceType::Wikidata,
    name: "AutoRoute GB File",
    extensions: &["axg"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
