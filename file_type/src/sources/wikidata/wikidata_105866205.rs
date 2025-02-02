use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105866205: FileFormat = FileFormat {
    id: 105_866_205,
    source_type: SourceType::Wikidata,
    name: "Image Packaging System Manifest (with rem)",
    extensions: &["p5m"],
    media_types: &["text/plain"],
    internal_signatures: &[],
    related_formats: &[],
};
