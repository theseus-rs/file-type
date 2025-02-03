use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_122412480: FileFormat = FileFormat {
    id: 122_412_480,
    source_type: SourceType::Wikidata,
    name: "Merge File",
    extensions: &["mer"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
