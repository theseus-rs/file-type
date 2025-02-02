use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_124969775: FileFormat = FileFormat {
    id: 124_969_775,
    source_type: SourceType::Wikidata,
    name: "Songsmith file",
    extensions: &["songsmith"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
