use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28206181: FileFormat = FileFormat {
    id: 28_206_181,
    source_type: SourceType::Wikidata,
    name: "GIMP Parametric Brush",
    extensions: &["vbr"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
