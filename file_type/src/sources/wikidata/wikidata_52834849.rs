use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_52834849: FileFormat = FileFormat {
    id: 52_834_849,
    source_type: SourceType::Wikidata,
    name: "dBASE for Windows database, version 5",
    extensions: &["dbf"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
