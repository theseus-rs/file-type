use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_100299731: FileFormat = FileFormat {
    id: 100_299_731,
    source_type: SourceType::Wikidata,
    name: "Flow Charting file format, version 6",
    extensions: &["fcx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
