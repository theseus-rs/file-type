use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_113556932: FileFormat = FileFormat {
    id: 113_556_932,
    source_type: SourceType::Wikidata,
    name: "Duplicator Image format",
    extensions: &["dao"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
