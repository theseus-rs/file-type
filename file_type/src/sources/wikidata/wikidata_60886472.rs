use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_60886472: FileFormat = FileFormat {
    id: 60_886_472,
    source_type: SourceType::Wikidata,
    name: "Drawing Interchange File Format (ASCII), version 1",
    extensions: &["dxf"],
    media_types: &["image/vnd.dxf"],
    signatures: &[],
    related_formats: &[],
};
