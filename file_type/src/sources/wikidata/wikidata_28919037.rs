use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28919037: FileFormat = FileFormat {
    id: 28_919_037,
    source_type: SourceType::Wikidata,
    name: "Type-2 DV AVI",
    extensions: &["avi"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
