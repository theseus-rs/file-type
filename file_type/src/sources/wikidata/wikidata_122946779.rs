use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_122946779: FileFormat = FileFormat {
    id: 122_946_779,
    source_type: SourceType::Wikidata,
    name: "Drawing Interchange File Format (ASCII), version 2004-2005",
    extensions: &["dxf"],
    media_types: &["image/vnd.dxf"],
    signatures: &[],
    related_formats: &[],
};
