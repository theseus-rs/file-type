use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_122248584: FileFormat = FileFormat {
    id: 122_248_584,
    source_type: SourceType::Wikidata,
    name: "YUV Video File",
    extensions: &["yuv"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
