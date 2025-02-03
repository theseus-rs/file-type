use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27355565: FileFormat = FileFormat {
    id: 27_355_565,
    source_type: SourceType::Wikidata,
    name: "ADRG Quality File",
    extensions: &["qal"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
