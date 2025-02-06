use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_81526237: FileFormat = FileFormat {
    id: 81_526_237,
    source_type: SourceType::Wikidata,
    name: "MapInfo MapBasic tabular DataBase",
    extensions: &["dat"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
