use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105861186: FileFormat = FileFormat {
    id: 105_861_186,
    source_type: SourceType::Wikidata,
    name: "X-Plane Painted Line",
    extensions: &["lin"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
