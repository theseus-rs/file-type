use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_95733139: FileFormat = FileFormat {
    id: 95_733_139,
    source_type: SourceType::Wikidata,
    name: "RealAudio version 3",
    extensions: &["ra"],
    media_types: &["audio/vnd.rn-realaudio"],
    signatures: &[],
    related_formats: &[],
};
