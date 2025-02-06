use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_96145366: FileFormat = FileFormat {
    id: 96_145_366,
    source_type: SourceType::Wikidata,
    name: "Wolfram Data Exchange format",
    extensions: &["wdx"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
