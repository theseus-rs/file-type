use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_51753051: FileFormat = FileFormat {
    id: 51_753_051,
    source_type: SourceType::Wikidata,
    name: "3D Studio Shapes",
    extensions: &["shp"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
