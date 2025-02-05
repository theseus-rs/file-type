use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_115055515: FileFormat = FileFormat {
    id: 115_055_515,
    source_type: SourceType::Wikidata,
    name: "The Spectral Geologist Dataset 7",
    extensions: &["tsg"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
