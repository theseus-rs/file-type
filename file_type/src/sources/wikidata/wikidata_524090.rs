use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_524090: FileFormat = FileFormat {
    id: 524_090,
    source_type: SourceType::Wikidata,
    name: "MT9",
    extensions: &["mt9"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
