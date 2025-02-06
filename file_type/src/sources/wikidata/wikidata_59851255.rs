use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_59851255: FileFormat = FileFormat {
    id: 59_851_255,
    source_type: SourceType::Wikidata,
    name: "Drawing Interchange File Format (ASCII)",
    extensions: &["dxf"],
    media_types: &["image/vnd.dxf"],
    signatures: &[],
    related_formats: &[],
};
