use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_96000078: FileFormat = FileFormat {
    id: 96_000_078,
    source_type: SourceType::Wikidata,
    name: "NOFF 3D geometry format",
    extensions: &["noff"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
