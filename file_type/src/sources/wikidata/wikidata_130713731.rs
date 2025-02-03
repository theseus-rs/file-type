use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_130713731: FileFormat = FileFormat {
    id: 130_713_731,
    source_type: SourceType::Wikidata,
    name: "RSL file format",
    extensions: &["rsl"],
    media_types: &["text/rsl"],
    internal_signatures: &[],
    related_formats: &[],
};
