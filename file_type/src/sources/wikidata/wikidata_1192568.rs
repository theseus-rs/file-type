use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_1192568: FileFormat = FileFormat {
    id: 1_192_568,
    source_type: SourceType::Wikidata,
    name: ".sys",
    extensions: &["sys"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
