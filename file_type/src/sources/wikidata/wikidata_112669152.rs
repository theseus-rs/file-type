use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_112669152: FileFormat = FileFormat {
    id: 112_669_152,
    source_type: SourceType::Wikidata,
    name: "Lightscape Parameter",
    extensions: &["df"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
