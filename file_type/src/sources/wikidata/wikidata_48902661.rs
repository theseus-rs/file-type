use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_48902661: FileFormat = FileFormat {
    id: 48_902_661,
    source_type: SourceType::Wikidata,
    name: "Harvard Graphics Presentation Slideshow",
    extensions: &["shw"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
