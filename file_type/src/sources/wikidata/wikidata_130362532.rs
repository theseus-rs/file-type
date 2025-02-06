use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_130362532: FileFormat = FileFormat {
    id: 130_362_532,
    source_type: SourceType::Wikidata,
    name: "MuPAD file format",
    extensions: &["mu"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
