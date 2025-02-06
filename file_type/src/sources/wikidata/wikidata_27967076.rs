use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27967076: FileFormat = FileFormat {
    id: 27_967_076,
    source_type: SourceType::Wikidata,
    name: "Audio Sculpture",
    extensions: &["adsc", "as"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
