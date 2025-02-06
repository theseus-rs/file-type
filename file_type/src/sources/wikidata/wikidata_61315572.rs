use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_61315572: FileFormat = FileFormat {
    id: 61_315_572,
    source_type: SourceType::Wikidata,
    name: "Drawing Interchange File Format (ASCII), version R9",
    extensions: &["dxf"],
    media_types: &["image/vnd.dxf"],
    signatures: &[],
    related_formats: &[],
};
