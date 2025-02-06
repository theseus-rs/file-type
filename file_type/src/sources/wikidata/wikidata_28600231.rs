use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28600231: FileFormat = FileFormat {
    id: 28_600_231,
    source_type: SourceType::Wikidata,
    name: "APW",
    extensions: &["apw"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
