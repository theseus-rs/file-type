use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27959824: FileFormat = FileFormat {
    id: 27_959_824,
    source_type: SourceType::Wikidata,
    name: "Ableton Skin File",
    extensions: &["ask"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
