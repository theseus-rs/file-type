use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_127817534: FileFormat = FileFormat {
    id: 127_817_534,
    source_type: SourceType::Wikidata,
    name: "gp script",
    extensions: &["gp"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
