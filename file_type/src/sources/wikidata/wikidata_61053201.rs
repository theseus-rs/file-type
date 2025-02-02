use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_61053201: FileFormat = FileFormat {
    id: 61_053_201,
    source_type: SourceType::Wikidata,
    name: "Drawing Interchange File Format (ASCII), version 1.3",
    extensions: &["dxf"],
    media_types: &["image/vnd.dxf"],
    internal_signatures: &[],
    related_formats: &[],
};
