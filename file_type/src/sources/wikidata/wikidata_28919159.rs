use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28919159: FileFormat = FileFormat {
    id: 28_919_159,
    source_type: SourceType::Wikidata,
    name: "Standard ACIS Text",
    extensions: &["sat"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
