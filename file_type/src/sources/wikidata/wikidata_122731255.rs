use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_122731255: FileFormat = FileFormat {
    id: 122_731_255,
    source_type: SourceType::Wikidata,
    name: "NCR G4 file format",
    extensions: &["ncr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
