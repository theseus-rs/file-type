use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_113544510: FileFormat = FileFormat {
    id: 113_544_510,
    source_type: SourceType::Wikidata,
    name: "PowerGraphics Image File",
    extensions: &["pgr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
