use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28018477: FileFormat = FileFormat {
    id: 28_018_477,
    source_type: SourceType::Wikidata,
    name: "Indeo Video Format",
    extensions: &["ivf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
