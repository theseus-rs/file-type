use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27967426: FileFormat = FileFormat {
    id: 27_967_426,
    source_type: SourceType::Wikidata,
    name: "Creative Music System",
    extensions: &["cms"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
