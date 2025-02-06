use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27967351: FileFormat = FileFormat {
    id: 27_967_351,
    source_type: SourceType::Wikidata,
    name: "iTunes Music Library, XML variant",
    extensions: &["xml"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
