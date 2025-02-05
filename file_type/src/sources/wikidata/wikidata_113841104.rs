use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_113841104: FileFormat = FileFormat {
    id: 113_841_104,
    source_type: SourceType::Wikidata,
    name: "Medi@Show Film File",
    extensions: &["flm"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
