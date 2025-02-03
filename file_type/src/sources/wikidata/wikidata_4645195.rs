use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_4645195: FileFormat = FileFormat {
    id: 4_645_195,
    source_type: SourceType::Wikidata,
    name: "8-Bit Sampled Voice",
    extensions: &["8svx", "iff"],
    media_types: &["audio/8svx", "audio/x-8svx"],
    internal_signatures: &[],
    related_formats: &[],
};
