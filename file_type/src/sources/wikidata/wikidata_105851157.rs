use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851157: FileFormat = FileFormat {
    id: 105_851_157,
    source_type: SourceType::Wikidata,
    name: "World of Warcraft TOC file",
    extensions: &["toc"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
