use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28551302: FileFormat = FileFormat {
    id: 28_551_302,
    source_type: SourceType::Wikidata,
    name: "Adobe Contour File",
    extensions: &["shc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
