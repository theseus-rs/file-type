use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_119443772: FileFormat = FileFormat {
    id: 119_443_772,
    source_type: SourceType::Wikidata,
    name: "AutoRoute File",
    extensions: &["axe"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
