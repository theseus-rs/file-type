use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_47196554: FileFormat = FileFormat {
    id: 47_196_554,
    source_type: SourceType::Wikidata,
    name: "AppleWorks Painting file format, version 5",
    extensions: &["cwk"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
