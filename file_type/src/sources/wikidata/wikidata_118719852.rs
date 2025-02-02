use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_118719852: FileFormat = FileFormat {
    id: 118_719_852,
    source_type: SourceType::Wikidata,
    name: "Poser 1.0 Pose Library",
    extensions: &["plb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
