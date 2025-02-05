use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28206252: FileFormat = FileFormat {
    id: 28_206_252,
    source_type: SourceType::Wikidata,
    name: "HMR",
    extensions: &["hmr"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
