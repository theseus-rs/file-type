use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_87896505: FileFormat = FileFormat {
    id: 87_896_505,
    source_type: SourceType::Wikidata,
    name: "Avery DesignPro Document 4",
    extensions: &["zdp"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
