use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105862425: FileFormat = FileFormat {
    id: 105_862_425,
    source_type: SourceType::Wikidata,
    name: "DASH Media Presentation Description",
    extensions: &["mpd"],
    media_types: &["application/dash+xml"],
    signatures: &[],
    related_formats: &[],
};
