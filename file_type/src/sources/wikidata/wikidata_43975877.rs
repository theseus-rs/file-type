use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_43975877: FileFormat = FileFormat {
    id: 43_975_877,
    source_type: SourceType::Wikidata,
    name: "Drawing Interchange File Format (Binary), version 2004-2005",
    extensions: &["dxf"],
    media_types: &["image/vnd.dxf"],
    signatures: &[],
    related_formats: &[],
};
