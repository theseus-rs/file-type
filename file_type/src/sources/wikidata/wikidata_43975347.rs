use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_43975347: FileFormat = FileFormat {
    id: 43_975_347,
    source_type: SourceType::Wikidata,
    name: "Drawing Interchange File Format (Binary), version R14",
    extensions: &["dxf"],
    media_types: &["image/vnd.dxf"],
    internal_signatures: &[],
    related_formats: &[],
};
