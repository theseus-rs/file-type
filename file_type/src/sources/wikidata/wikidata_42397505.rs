use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_42397505: FileFormat = FileFormat {
    id: 42_397_505,
    source_type: SourceType::Wikidata,
    name: "vimwiki",
    extensions: &["wiki"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
