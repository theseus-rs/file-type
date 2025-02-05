use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_131747923: FileFormat = FileFormat {
    id: 131_747_923,
    source_type: SourceType::Wikidata,
    name: "MNI polygonal surface mesh format",
    extensions: &["obj"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
