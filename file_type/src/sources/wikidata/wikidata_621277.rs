use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_621277: FileFormat = FileFormat {
    id: 621_277,
    source_type: SourceType::Wikidata,
    name: "Apple Lossless",
    extensions: &["caf", "m4a"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
