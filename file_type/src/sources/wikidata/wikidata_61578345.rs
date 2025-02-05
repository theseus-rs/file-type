use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_61578345: FileFormat = FileFormat {
    id: 61_578_345,
    source_type: SourceType::Wikidata,
    name: "Drawing Interchange File Format (ASCII), version 2000-2002",
    extensions: &["dxf"],
    media_types: &["image/vnd.dxf"],
    signatures: &[],
    related_formats: &[],
};
