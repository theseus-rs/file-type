use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_118465354: FileFormat = FileFormat {
    id: 118_465_354,
    source_type: SourceType::Wikidata,
    name: "Capture One Session File",
    extensions: &["cos"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
