use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_122974666: FileFormat = FileFormat {
    id: 122_974_666,
    source_type: SourceType::Wikidata,
    name: "CAMP",
    extensions: &["camp"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
