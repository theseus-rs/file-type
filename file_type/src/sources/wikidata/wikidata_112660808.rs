use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_112660808: FileFormat = FileFormat {
    id: 112_660_808,
    source_type: SourceType::Wikidata,
    name: "MediaView file",
    extensions: &["m14"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
