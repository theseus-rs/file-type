use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_122333759: FileFormat = FileFormat {
    id: 122_333_759,
    source_type: SourceType::Wikidata,
    name: "Logo Design Studio File",
    extensions: &["lds"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
