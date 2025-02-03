use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_95733178: FileFormat = FileFormat {
    id: 95_733_178,
    source_type: SourceType::Wikidata,
    name: "RealAudio version 4",
    extensions: &["ra"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
