use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27996244: FileFormat = FileFormat {
    id: 27_996_244,
    source_type: SourceType::Wikidata,
    name: "HyperCard stack",
    extensions: &["pdf", "tif"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
