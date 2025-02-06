use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_120965459: FileFormat = FileFormat {
    id: 120_965_459,
    source_type: SourceType::Wikidata,
    name: "Microsoft Money version 3 data",
    extensions: &["mn3"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
