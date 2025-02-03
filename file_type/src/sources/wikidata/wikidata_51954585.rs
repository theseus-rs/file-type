use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_51954585: FileFormat = FileFormat {
    id: 51_954_585,
    source_type: SourceType::Wikidata,
    name: "dBASE III+ file format",
    extensions: &["dbf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
