use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_113376526: FileFormat = FileFormat {
    id: 113_376_526,
    source_type: SourceType::Wikidata,
    name: "RED Thumbnail File",
    extensions: &["rtn"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
