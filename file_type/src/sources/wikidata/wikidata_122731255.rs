use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_122731255: FileFormat = FileFormat {
    id: 122_731_255,
    source_type: SourceType::Wikidata,
    name: "NCR G4 file format",
    extensions: &["ncr"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
