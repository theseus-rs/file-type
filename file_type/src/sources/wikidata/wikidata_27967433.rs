use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27967433: FileFormat = FileFormat {
    id: 27_967_433,
    source_type: SourceType::Wikidata,
    name: "Bink Video, version 1",
    extensions: &["bik"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
