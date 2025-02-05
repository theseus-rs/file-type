use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_61053371: FileFormat = FileFormat {
    id: 61_053_371,
    source_type: SourceType::Wikidata,
    name: "Drawing Interchange File Format (ASCII), version 1.4",
    extensions: &["dxf"],
    media_types: &["image/vnd.dxf"],
    signatures: &[],
    related_formats: &[],
};
