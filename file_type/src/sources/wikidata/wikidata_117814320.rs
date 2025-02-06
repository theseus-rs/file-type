use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_117814320: FileFormat = FileFormat {
    id: 117_814_320,
    source_type: SourceType::Wikidata,
    name: "Working Model 3D Document",
    extensions: &["wm3"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
