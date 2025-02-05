use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27823194: FileFormat = FileFormat {
    id: 27_823_194,
    source_type: SourceType::Wikidata,
    name: "Binary Terrain, version 1.2",
    extensions: &["bt"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
