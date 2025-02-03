use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_116323728: FileFormat = FileFormat {
    id: 116_323_728,
    source_type: SourceType::Wikidata,
    name: "Photosuite Album File",
    extensions: &["pza"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
