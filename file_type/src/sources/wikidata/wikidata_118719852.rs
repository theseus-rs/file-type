use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_118719852: FileFormat = FileFormat {
    id: 118_719_852,
    source_type: SourceType::Wikidata,
    name: "Poser 1.0 Pose Library",
    extensions: &["plb"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
