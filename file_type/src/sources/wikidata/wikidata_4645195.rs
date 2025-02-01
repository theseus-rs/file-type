use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_4645195: FileFormat = FileFormat {
    id: 4_645_195,
    puid: "wikidata/4645195",
    name: "8-Bit Sampled Voice",
    extensions: &["8svx", "8svx", "iff", "iff"],
    media_types: &["audio/8svx", "audio/8svx", "audio/x-8svx", "audio/x-8svx"],
    internal_signatures: &[],
    related_formats: &[],
};
