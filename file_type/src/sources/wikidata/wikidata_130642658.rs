use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_130642658: FileFormat = FileFormat {
    id: 130_642_658,
    source_type: SourceType::Wikidata,
    name: "Robot Framework file format",
    extensions: &["robot"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
