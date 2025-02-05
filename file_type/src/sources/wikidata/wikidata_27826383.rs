use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27826383: FileFormat = FileFormat {
    id: 27_826_383,
    source_type: SourceType::Wikidata,
    name: "Reduced resolution dataset external file",
    extensions: &["rde"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
