use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_115055414: FileFormat = FileFormat {
    id: 115_055_414,
    source_type: SourceType::Wikidata,
    name: "The Spectral Geologist Dataset",
    extensions: &["tsg"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
