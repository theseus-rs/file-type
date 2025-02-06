use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_112653501: FileFormat = FileFormat {
    id: 112_653_501,
    source_type: SourceType::Wikidata,
    name: "Professional Draw 1.0 backup file",
    extensions: &["pd~"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
