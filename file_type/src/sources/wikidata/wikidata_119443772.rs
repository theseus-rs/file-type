use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_119443772: FileFormat = FileFormat {
    id: 119_443_772,
    source_type: SourceType::Wikidata,
    name: "AutoRoute File",
    extensions: &["axe"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
