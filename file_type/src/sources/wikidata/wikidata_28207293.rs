use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28207293: FileFormat = FileFormat {
    id: 28_207_293,
    source_type: SourceType::Wikidata,
    name: "Softimage PIC",
    extensions: &["pic"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
