use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27203404: FileFormat = FileFormat {
    id: 27_203_404,
    source_type: SourceType::Wikidata,
    name: "OpenDocument Text, version 1.1",
    extensions: &["odt"],
    media_types: &["application/vnd.oasis.opendocument.text"],
    internal_signatures: &[],
    related_formats: &[],
};
