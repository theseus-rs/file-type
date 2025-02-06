use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28600238: FileFormat = FileFormat {
    id: 28_600_238,
    source_type: SourceType::Wikidata,
    name: "ARC",
    extensions: &["arc"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
