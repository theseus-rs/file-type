use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_47012439: FileFormat = FileFormat {
    id: 47_012_439,
    source_type: SourceType::Wikidata,
    name: "Microsoft Works Document file format",
    extensions: &["bps"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
