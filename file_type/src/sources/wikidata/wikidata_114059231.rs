use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_114059231: FileFormat = FileFormat {
    id: 114_059_231,
    source_type: SourceType::Wikidata,
    name: "OpenDocument Graphics, version 1.3",
    extensions: &["odg"],
    media_types: &["application/vnd.oasis.opendocument.graphics"],
    signatures: &[],
    related_formats: &[],
};
