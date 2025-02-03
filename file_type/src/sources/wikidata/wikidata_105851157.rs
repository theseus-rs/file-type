use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105851157: FileFormat = FileFormat {
    id: 105_851_157,
    source_type: SourceType::Wikidata,
    name: "World of Warcraft TOC file",
    extensions: &["toc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
