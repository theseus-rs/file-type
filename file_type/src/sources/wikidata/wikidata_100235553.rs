use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_100235553: FileFormat = FileFormat {
    id: 100_235_553,
    source_type: SourceType::Wikidata,
    name: "FARO Laser Scan File",
    extensions: &["fls"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
