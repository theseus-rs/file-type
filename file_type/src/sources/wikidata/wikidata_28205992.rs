use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28205992: FileFormat = FileFormat {
    id: 28_205_992,
    source_type: SourceType::Wikidata,
    name: "Digital Video Interactive Alpha Channel",
    extensions: &["ima"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
