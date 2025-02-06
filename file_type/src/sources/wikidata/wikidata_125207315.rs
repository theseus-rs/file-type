use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_125207315: FileFormat = FileFormat {
    id: 125_207_315,
    source_type: SourceType::Wikidata,
    name: "VYM part",
    extensions: &["vyp"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
