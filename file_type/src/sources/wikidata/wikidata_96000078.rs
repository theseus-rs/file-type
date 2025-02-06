use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_96000078: FileFormat = FileFormat {
    id: 96_000_078,
    source_type: SourceType::Wikidata,
    name: "NOFF 3D geometry format",
    extensions: &["noff"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
