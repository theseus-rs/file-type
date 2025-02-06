use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_100304054: FileFormat = FileFormat {
    id: 100_304_054,
    source_type: SourceType::Wikidata,
    name: "Flow Charting Graphic Flowcharting Image",
    extensions: &["gfi"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
