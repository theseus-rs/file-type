use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_919226: FileFormat = FileFormat {
    id: 919_226,
    source_type: SourceType::Wikidata,
    name: "MPEG-1 Audio Layer I",
    extensions: &["m1a", "mp1"],
    media_types: &["audio/MPA", "audio/mpeg"],
    internal_signatures: &[],
    related_formats: &[],
};
