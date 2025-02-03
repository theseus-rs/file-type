use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_117022113: FileFormat = FileFormat {
    id: 117_022_113,
    source_type: SourceType::Wikidata,
    name: "Garden File",
    extensions: &["grd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
