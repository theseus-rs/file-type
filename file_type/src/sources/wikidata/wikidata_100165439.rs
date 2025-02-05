use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_100165439: FileFormat = FileFormat {
    id: 100_165_439,
    source_type: SourceType::Wikidata,
    name: "Drawing Interchange Format (Binary), version 2007-2009",
    extensions: &["dxf"],
    media_types: &["image/vnd.dxf"],
    signatures: &[],
    related_formats: &[],
};
