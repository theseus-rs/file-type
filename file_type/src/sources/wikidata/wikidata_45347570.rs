use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_45347570: FileFormat = FileFormat {
    id: 45_347_570,
    source_type: SourceType::Wikidata,
    name: "Lotus 1-2-3 Worksheet file format, version 4-5",
    extensions: &["wk4"],
    media_types: &["application/lotus123", "application/vnd.lotus-1-2-3"],
    signatures: &[],
    related_formats: &[],
};
