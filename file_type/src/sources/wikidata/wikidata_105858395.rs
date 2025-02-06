use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858395: FileFormat = FileFormat {
    id: 105_858_395,
    source_type: SourceType::Wikidata,
    name: "EAGLE script",
    extensions: &["ulp"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
