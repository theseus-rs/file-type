use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105862425: FileFormat = FileFormat {
    id: 105_862_425,
    source_type: SourceType::Wikidata,
    name: "DASH Media Presentation Description",
    extensions: &["mpd"],
    media_types: &["application/dash+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
