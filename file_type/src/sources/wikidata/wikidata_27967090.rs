use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27967090: FileFormat = FileFormat {
    id: 27_967_090,
    source_type: SourceType::Wikidata,
    name: "Epic Megagames MASI",
    extensions: &["psm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
