use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_61576485: FileFormat = FileFormat {
    id: 61_576_485,
    source_type: SourceType::Wikidata,
    name: "Drawing Interchange File Format (ASCII), version R13",
    extensions: &["dxf"],
    media_types: &["image/vnd.dxf"],
    signatures: &[],
    related_formats: &[],
};
