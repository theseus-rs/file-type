use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28205965: FileFormat = FileFormat {
    id: 28_205_965,
    source_type: SourceType::Wikidata,
    name: "Digital Video Interactive Green Channel",
    extensions: &["img"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
