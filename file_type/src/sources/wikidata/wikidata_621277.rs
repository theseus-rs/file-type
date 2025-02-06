use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_621277: FileFormat = FileFormat {
    id: 621_277,
    source_type: SourceType::Wikidata,
    name: "Apple Lossless",
    extensions: &["caf", "m4a"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
