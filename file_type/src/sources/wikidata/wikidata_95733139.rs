use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_95733139: FileFormat = FileFormat {
    id: 95_733_139,
    source_type: SourceType::Wikidata,
    name: "RealAudio version 3",
    extensions: &["ra"],
    media_types: &["audio/vnd.rn-realaudio"],
    internal_signatures: &[],
    related_formats: &[],
};
