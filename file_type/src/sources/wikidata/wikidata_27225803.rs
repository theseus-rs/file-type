use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27225803: FileFormat = FileFormat {
    id: 27_225_803,
    source_type: SourceType::Wikidata,
    name: "OpenDocument Database, version 1.0",
    extensions: &["odb"],
    media_types: &["application/vnd.oasis.opendocument.database"],
    internal_signatures: &[],
    related_formats: &[],
};
