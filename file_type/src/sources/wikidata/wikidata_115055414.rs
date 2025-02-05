use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_115055414: FileFormat = FileFormat {
    id: 115_055_414,
    source_type: SourceType::Wikidata,
    name: "The Spectral Geologist Dataset",
    extensions: &["tsg"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
