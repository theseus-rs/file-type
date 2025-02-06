use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_118583820: FileFormat = FileFormat {
    id: 118_583_820,
    source_type: SourceType::Wikidata,
    name: "Cakewalk Project",
    extensions: &["cwp"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
