use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28551401: FileFormat = FileFormat {
    id: 28_551_401,
    source_type: SourceType::Wikidata,
    name: "Adobe Separation Table File",
    extensions: &["ast"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
