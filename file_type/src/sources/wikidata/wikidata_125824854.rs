use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_125824854: FileFormat = FileFormat {
    id: 125_824_854,
    source_type: SourceType::Wikidata,
    name: "Tar Zipped File",
    extensions: &["taz"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
