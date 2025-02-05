use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_119443865: FileFormat = FileFormat {
    id: 119_443_865,
    source_type: SourceType::Wikidata,
    name: "AutoRoute GB File",
    extensions: &["axg"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
