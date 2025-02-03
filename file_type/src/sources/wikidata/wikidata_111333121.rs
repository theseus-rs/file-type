use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111333121: FileFormat = FileFormat {
    id: 111_333_121,
    source_type: SourceType::Wikidata,
    name: "OKI MSM6376 synth chip PCM format",
    extensions: &["pcm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
