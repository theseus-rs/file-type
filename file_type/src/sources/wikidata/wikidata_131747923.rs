use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_131747923: FileFormat = FileFormat {
    id: 131_747_923,
    source_type: SourceType::Wikidata,
    name: "MNI polygonal surface mesh format",
    extensions: &["obj"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
