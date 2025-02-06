use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_61692923: FileFormat = FileFormat {
    id: 61_692_923,
    source_type: SourceType::Wikidata,
    name: "Drawing Interchange File Format (Binary), version R10",
    extensions: &["dxf"],
    media_types: &["image/vnd.dxf"],
    signatures: &[],
    related_formats: &[],
};
