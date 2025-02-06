use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_130283788: FileFormat = FileFormat {
    id: 130_283_788,
    source_type: SourceType::Wikidata,
    name: "Maxima file format",
    extensions: &["mac", "max"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
