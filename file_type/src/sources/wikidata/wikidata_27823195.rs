use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27823195: FileFormat = FileFormat {
    id: 27_823_195,
    source_type: SourceType::Wikidata,
    name: "Binary Terrain External Projection file",
    extensions: &["prj"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
