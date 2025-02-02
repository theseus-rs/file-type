use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_116145260: FileFormat = FileFormat {
    id: 116_145_260,
    source_type: SourceType::Wikidata,
    name: "FIT file",
    extensions: &["fit"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
