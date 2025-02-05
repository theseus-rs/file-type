use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_47018292: FileFormat = FileFormat {
    id: 47_018_292,
    source_type: SourceType::Wikidata,
    name: "PageMaker Document file format, version 3",
    extensions: &["pm3"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
