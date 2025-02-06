use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_119582504: FileFormat = FileFormat {
    id: 119_582_504,
    source_type: SourceType::Wikidata,
    name: "EMLX",
    extensions: &["emlx"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
