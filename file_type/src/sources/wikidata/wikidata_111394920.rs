use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111394920: FileFormat = FileFormat {
    id: 111_394_920,
    source_type: SourceType::Wikidata,
    name: "Form File",
    extensions: &["fif"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
