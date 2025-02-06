use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105860673: FileFormat = FileFormat {
    id: 105_860_673,
    source_type: SourceType::Wikidata,
    name: "RealTime subtitles",
    extensions: &["rt"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
