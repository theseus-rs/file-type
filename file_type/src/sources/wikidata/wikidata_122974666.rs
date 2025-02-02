use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_122974666: FileFormat = FileFormat {
    id: 122_974_666,
    source_type: SourceType::Wikidata,
    name: "CAMP",
    extensions: &["camp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
