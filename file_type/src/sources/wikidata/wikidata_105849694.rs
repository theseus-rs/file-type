use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105849694: FileFormat = FileFormat {
    id: 105_849_694,
    source_type: SourceType::Wikidata,
    name: "Celestia script",
    extensions: &["cel"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
