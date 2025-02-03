use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_87896505: FileFormat = FileFormat {
    id: 87_896_505,
    source_type: SourceType::Wikidata,
    name: "Avery DesignPro Document 4",
    extensions: &["zdp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
