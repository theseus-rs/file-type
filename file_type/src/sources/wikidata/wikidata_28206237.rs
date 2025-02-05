use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28206237: FileFormat = FileFormat {
    id: 28_206_237,
    source_type: SourceType::Wikidata,
    name: "GROB",
    extensions: &["grb", "gro"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
