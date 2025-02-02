use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_47165600: FileFormat = FileFormat {
    id: 47_165_600,
    source_type: SourceType::Wikidata,
    name: "RealLegal E-Transcript file format",
    extensions: &["ptx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
