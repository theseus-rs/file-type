use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_113626396: FileFormat = FileFormat {
    id: 113_626_396,
    source_type: SourceType::Wikidata,
    name: "ScatterShow file",
    extensions: &["scattershow"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
