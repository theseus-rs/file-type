use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_117485947: FileFormat = FileFormat {
    id: 117_485_947,
    source_type: SourceType::Wikidata,
    name: "Audacity Project File 2.x",
    extensions: &["aup"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
