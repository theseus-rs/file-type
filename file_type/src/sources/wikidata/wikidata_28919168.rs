use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28919168: FileFormat = FileFormat {
    id: 28_919_168,
    source_type: SourceType::Wikidata,
    name: "GHS Part Maker",
    extensions: &["pm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
