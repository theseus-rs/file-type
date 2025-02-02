use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111009551: FileFormat = FileFormat {
    id: 111_009_551,
    source_type: SourceType::Wikidata,
    name: "PrintMaster Craft File format",
    extensions: &["cft"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
