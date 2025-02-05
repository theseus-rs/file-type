use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28551347: FileFormat = FileFormat {
    id: 28_551_347,
    source_type: SourceType::Wikidata,
    name: "Adobe Halftone Screens File",
    extensions: &["ahs"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
