use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28600238: FileFormat = FileFormat {
    id: 28_600_238,
    source_type: SourceType::Wikidata,
    name: "ARC",
    extensions: &["arc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
