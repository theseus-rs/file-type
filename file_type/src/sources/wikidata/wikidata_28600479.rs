use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28600479: FileFormat = FileFormat {
    id: 28_600_479,
    source_type: SourceType::Wikidata,
    name: "DOTX",
    extensions: &["dotx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
