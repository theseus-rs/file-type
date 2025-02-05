use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_5299371: FileFormat = FileFormat {
    id: 5_299_371,
    source_type: SourceType::Wikidata,
    name: "dotXSI",
    extensions: &["xsi"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
