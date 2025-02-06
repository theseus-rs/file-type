use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111333121: FileFormat = FileFormat {
    id: 111_333_121,
    source_type: SourceType::Wikidata,
    name: "OKI MSM6376 synth chip PCM format",
    extensions: &["pcm"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
