use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_113841104: FileFormat = FileFormat {
    id: 113_841_104,
    source_type: SourceType::Wikidata,
    name: "Medi@Show Film File",
    extensions: &["flm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
