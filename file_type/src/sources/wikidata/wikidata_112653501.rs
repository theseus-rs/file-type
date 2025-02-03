use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_112653501: FileFormat = FileFormat {
    id: 112_653_501,
    source_type: SourceType::Wikidata,
    name: "Professional Draw 1.0 backup file",
    extensions: &["pd~"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
