use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_116323728: FileFormat = FileFormat {
    id: 116_323_728,
    source_type: SourceType::Wikidata,
    name: "Photosuite Album File",
    extensions: &["pza"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
