use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105849694: FileFormat = FileFormat {
    id: 105_849_694,
    source_type: SourceType::Wikidata,
    name: "Celestia script",
    extensions: &["cel"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
