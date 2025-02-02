use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_184473: FileFormat = FileFormat {
    id: 184_473,
    source_type: SourceType::Wikidata,
    name: "OpenDocument",
    extensions: &["fodt", "odt"],
    media_types: &["application/vnd.oasis.opendocument.text"],
    internal_signatures: &[],
    related_formats: &[],
};
