use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_61576757: FileFormat = FileFormat {
    id: 61_576_757,
    source_type: SourceType::Wikidata,
    name: "Drawing Interchange File Format (ASCII), version R14",
    extensions: &["dxf"],
    media_types: &["image/vnd.dxf"],
    internal_signatures: &[],
    related_formats: &[],
};
