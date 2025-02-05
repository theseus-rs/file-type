use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28600390: FileFormat = FileFormat {
    id: 28_600_390,
    source_type: SourceType::Wikidata,
    name: "Apple framework",
    extensions: &["framework"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
