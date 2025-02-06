use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27959833: FileFormat = FileFormat {
    id: 27_959_833,
    source_type: SourceType::Wikidata,
    name: "Cool Edit/Audition Multi Track Session file",
    extensions: &["ses"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
