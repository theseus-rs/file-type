use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_116804559: FileFormat = FileFormat {
    id: 116_804_559,
    source_type: SourceType::Wikidata,
    name: "TimeWiz Project File",
    extensions: &["twz"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
