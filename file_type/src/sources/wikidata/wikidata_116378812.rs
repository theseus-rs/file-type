use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_116378812: FileFormat = FileFormat {
    id: 116_378_812,
    source_type: SourceType::Wikidata,
    name: "Act! Database File",
    extensions: &["dbf"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
