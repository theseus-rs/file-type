use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_47018292: FileFormat = FileFormat {
    id: 47_018_292,
    source_type: SourceType::Wikidata,
    name: "PageMaker Document file format, version 3",
    extensions: &["pm3"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
