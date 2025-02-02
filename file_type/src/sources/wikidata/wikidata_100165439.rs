use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_100165439: FileFormat = FileFormat {
    id: 100_165_439,
    source_type: SourceType::Wikidata,
    name: "Drawing Interchange Format (Binary), version 2007-2009",
    extensions: &["dxf"],
    media_types: &["image/vnd.dxf"],
    internal_signatures: &[],
    related_formats: &[],
};
