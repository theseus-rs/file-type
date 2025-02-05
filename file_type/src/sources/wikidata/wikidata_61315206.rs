use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_61315206: FileFormat = FileFormat {
    id: 61_315_206,
    source_type: SourceType::Wikidata,
    name: "Drawing Interchange File Format (ASCII), version 2.5",
    extensions: &["dxf"],
    media_types: &["image/vnd.dxf"],
    signatures: &[],
    related_formats: &[],
};
