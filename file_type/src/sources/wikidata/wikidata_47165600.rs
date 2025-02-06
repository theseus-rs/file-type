use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_47165600: FileFormat = FileFormat {
    id: 47_165_600,
    source_type: SourceType::Wikidata,
    name: "RealLegal E-Transcript file format",
    extensions: &["ptx"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
