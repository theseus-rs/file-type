use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111182275: FileFormat = FileFormat {
    id: 111_182_275,
    source_type: SourceType::Wikidata,
    name: "ActionScript Remote File",
    extensions: &["asr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
