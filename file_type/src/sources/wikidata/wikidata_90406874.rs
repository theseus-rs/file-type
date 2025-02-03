use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_90406874: FileFormat = FileFormat {
    id: 90_406_874,
    source_type: SourceType::Wikidata,
    name: "QuickTake format",
    extensions: &["qtk"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
