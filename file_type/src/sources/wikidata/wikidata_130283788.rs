use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_130283788: FileFormat = FileFormat {
    id: 130_283_788,
    source_type: SourceType::Wikidata,
    name: "Maxima file format",
    extensions: &["mac", "max"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
