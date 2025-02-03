use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_115055515: FileFormat = FileFormat {
    id: 115_055_515,
    source_type: SourceType::Wikidata,
    name: "The Spectral Geologist Dataset 7",
    extensions: &["tsg"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
