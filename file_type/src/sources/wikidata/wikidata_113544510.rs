use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_113544510: FileFormat = FileFormat {
    id: 113_544_510,
    source_type: SourceType::Wikidata,
    name: "PowerGraphics Image File",
    extensions: &["pgr"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
