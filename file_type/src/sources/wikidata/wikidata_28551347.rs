use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28551347: FileFormat = FileFormat {
    id: 28_551_347,
    source_type: SourceType::Wikidata,
    name: "Adobe Halftone Screens File",
    extensions: &["ahs"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
