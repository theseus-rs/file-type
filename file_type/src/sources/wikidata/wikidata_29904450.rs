use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_29904450: FileFormat = FileFormat {
    id: 29_904_450,
    source_type: SourceType::Wikidata,
    name: "Presentation Manager Metafile",
    extensions: &["met"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
