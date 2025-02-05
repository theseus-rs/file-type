use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28975794: FileFormat = FileFormat {
    id: 28_975_794,
    source_type: SourceType::Wikidata,
    name: "Drawing Interchange File Format (ASCII), version R11/R12",
    extensions: &["dxf"],
    media_types: &["image/vnd.dxf"],
    signatures: &[],
    related_formats: &[],
};
