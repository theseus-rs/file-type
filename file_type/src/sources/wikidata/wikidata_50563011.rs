use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_50563011: FileFormat = FileFormat {
    id: 50_563_011,
    source_type: SourceType::Wikidata,
    name: "BKNAS Seismic Data Format",
    extensions: &["bknas"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
