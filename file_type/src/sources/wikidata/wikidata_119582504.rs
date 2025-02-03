use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_119582504: FileFormat = FileFormat {
    id: 119_582_504,
    source_type: SourceType::Wikidata,
    name: "EMLX",
    extensions: &["emlx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
