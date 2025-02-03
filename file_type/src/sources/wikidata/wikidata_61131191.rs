use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_61131191: FileFormat = FileFormat {
    id: 61_131_191,
    source_type: SourceType::Wikidata,
    name: "Drawing Interchange File Format (ASCII), version 2",
    extensions: &["dxf"],
    media_types: &["image/vnd.dxf"],
    internal_signatures: &[],
    related_formats: &[],
};
