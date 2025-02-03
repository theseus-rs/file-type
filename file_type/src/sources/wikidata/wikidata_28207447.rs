use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28207447: FileFormat = FileFormat {
    id: 28_207_447,
    source_type: SourceType::Wikidata,
    name: "VIPS",
    extensions: &["v"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
