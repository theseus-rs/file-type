use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_919226: FileFormat = FileFormat {
    id: 919_226,
    source_type: SourceType::Wikidata,
    name: "MPEG-1 Audio Layer I",
    extensions: &["m1a", "mp1"],
    media_types: &["audio/MPA", "audio/mpeg"],
    signatures: &[],
    related_formats: &[],
};
