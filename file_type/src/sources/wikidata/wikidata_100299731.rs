use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_100299731: FileFormat = FileFormat {
    id: 100_299_731,
    source_type: SourceType::Wikidata,
    name: "Flow Charting file format, version 6",
    extensions: &["fcx"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
