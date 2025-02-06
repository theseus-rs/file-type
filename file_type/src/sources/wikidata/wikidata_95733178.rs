use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_95733178: FileFormat = FileFormat {
    id: 95_733_178,
    source_type: SourceType::Wikidata,
    name: "RealAudio version 4",
    extensions: &["ra"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
