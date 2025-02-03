use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_118583820: FileFormat = FileFormat {
    id: 118_583_820,
    source_type: SourceType::Wikidata,
    name: "Cakewalk Project",
    extensions: &["cwp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
