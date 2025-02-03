use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28919037: FileFormat = FileFormat {
    id: 28_919_037,
    source_type: SourceType::Wikidata,
    name: "Type-2 DV AVI",
    extensions: &["avi"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
