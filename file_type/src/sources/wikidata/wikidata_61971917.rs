use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_61971917: FileFormat = FileFormat {
    id: 61_971_917,
    source_type: SourceType::Wikidata,
    name: "FoxPro Database, version 2x",
    extensions: &["dbf"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
