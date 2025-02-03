use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105860673: FileFormat = FileFormat {
    id: 105_860_673,
    source_type: SourceType::Wikidata,
    name: "RealTime subtitles",
    extensions: &["rt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
