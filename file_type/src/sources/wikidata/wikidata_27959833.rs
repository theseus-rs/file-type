use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27959833: FileFormat = FileFormat {
    id: 27_959_833,
    source_type: SourceType::Wikidata,
    name: "Cool Edit/Audition Multi Track Session file",
    extensions: &["ses"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
