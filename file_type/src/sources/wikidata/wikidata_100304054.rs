use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_100304054: FileFormat = FileFormat {
    id: 100_304_054,
    source_type: SourceType::Wikidata,
    name: "Flow Charting Graphic Flowcharting Image",
    extensions: &["gfi"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
