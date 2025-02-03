use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28206181: FileFormat = FileFormat {
    id: 28_206_181,
    source_type: SourceType::Wikidata,
    name: "GIMP Parametric Brush",
    extensions: &["vbr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
