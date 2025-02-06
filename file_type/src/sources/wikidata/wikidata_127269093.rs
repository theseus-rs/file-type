use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_127269093: FileFormat = FileFormat {
    id: 127_269_093,
    source_type: SourceType::Wikidata,
    name: "Bulk Data File",
    extensions: &["bdf"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
