use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_130642658: FileFormat = FileFormat {
    id: 130_642_658,
    source_type: SourceType::Wikidata,
    name: "Robot Framework file format",
    extensions: &["robot"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
