use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111333121: FileFormat = FileFormat {
    id: 111_333_121,
    puid: "wikidata/111333121",
    name: "OKI MSM6376 synth chip PCM format",
    extensions: &["pcm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
