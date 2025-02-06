use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27225813: FileFormat = FileFormat {
    id: 27_225_813,
    source_type: SourceType::Wikidata,
    name: "OpenDocument Database, version 1.2",
    extensions: &["odb"],
    media_types: &["application/vnd.oasis.opendocument.database"],
    signatures: &[],
    related_formats: &[],
};
