use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_125824854: FileFormat = FileFormat {
    id: 125_824_854,
    source_type: SourceType::Wikidata,
    name: "Tar Zipped File",
    extensions: &["taz"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
