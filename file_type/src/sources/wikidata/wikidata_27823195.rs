use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27823195: FileFormat = FileFormat {
    id: 27_823_195,
    source_type: SourceType::Wikidata,
    name: "Binary Terrain External Projection file",
    extensions: &["prj"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
