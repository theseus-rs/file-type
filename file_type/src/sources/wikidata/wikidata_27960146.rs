use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27960146: FileFormat = FileFormat {
    id: 27_960_146,
    source_type: SourceType::Wikidata,
    name: "X2A",
    extensions: &["x2a"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
