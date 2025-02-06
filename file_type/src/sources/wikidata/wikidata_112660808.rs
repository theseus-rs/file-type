use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_112660808: FileFormat = FileFormat {
    id: 112_660_808,
    source_type: SourceType::Wikidata,
    name: "MediaView file",
    extensions: &["m14"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
