use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_114059139: FileFormat = FileFormat {
    id: 114_059_139,
    source_type: SourceType::Wikidata,
    name: "OpenDocument Database, version 1.3",
    extensions: &["odb"],
    media_types: &["application/vnd.oasis.opendocument.base"],
    signatures: &[],
    related_formats: &[],
};
