use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_32097899: FileFormat = FileFormat {
    id: 32_097_899,
    source_type: SourceType::Wikidata,
    name: "Fallout v2 DAT",
    extensions: &["dat"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
