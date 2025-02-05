use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_47019464: FileFormat = FileFormat {
    id: 47_019_464,
    source_type: SourceType::Wikidata,
    name: "PageMaker Document file format, version 5",
    extensions: &["pm5"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
