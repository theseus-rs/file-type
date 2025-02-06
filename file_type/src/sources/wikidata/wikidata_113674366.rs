use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_113674366: FileFormat = FileFormat {
    id: 113_674_366,
    source_type: SourceType::Wikidata,
    name: "Final Draft 5-7 Template",
    extensions: &["fdt"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
