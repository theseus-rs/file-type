use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_47019065: FileFormat = FileFormat {
    id: 47_019_065,
    source_type: SourceType::Wikidata,
    name: "PageMaker Document file format, version 6",
    extensions: &["pm6"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
