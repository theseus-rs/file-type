use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_125915605: FileFormat = FileFormat {
    id: 125_915_605,
    source_type: SourceType::Wikidata,
    name: "Melco OFM Project pre v.11",
    extensions: &["ofm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
