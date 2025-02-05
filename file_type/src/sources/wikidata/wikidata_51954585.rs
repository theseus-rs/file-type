use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_51954585: FileFormat = FileFormat {
    id: 51_954_585,
    source_type: SourceType::Wikidata,
    name: "dBASE III+ file format",
    extensions: &["dbf"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
