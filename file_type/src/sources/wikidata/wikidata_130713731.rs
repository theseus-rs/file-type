use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_130713731: FileFormat = FileFormat {
    id: 130_713_731,
    source_type: SourceType::Wikidata,
    name: "RSL file format",
    extensions: &["rsl"],
    media_types: &["text/rsl"],
    signatures: &[],
    related_formats: &[],
};
