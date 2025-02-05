use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_128792608: FileFormat = FileFormat {
    id: 128_792_608,
    source_type: SourceType::Wikidata,
    name: "DAX formula file",
    extensions: &["dax"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
