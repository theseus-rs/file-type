use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_100165480: FileFormat = FileFormat {
    id: 100_165_480,
    source_type: SourceType::Wikidata,
    name: "Drawing Interchange Format (Binary), version 2010-2012",
    extensions: &["dxf"],
    media_types: &["image/vnd.dxf"],
    signatures: &[],
    related_formats: &[],
};
