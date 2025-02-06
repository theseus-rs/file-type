use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28551302: FileFormat = FileFormat {
    id: 28_551_302,
    source_type: SourceType::Wikidata,
    name: "Adobe Contour File",
    extensions: &["shc"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
