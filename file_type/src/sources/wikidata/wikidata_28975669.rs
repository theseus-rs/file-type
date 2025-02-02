use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28975669: FileFormat = FileFormat {
    id: 28_975_669,
    source_type: SourceType::Wikidata,
    name: "BMF",
    extensions: &["bmf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
