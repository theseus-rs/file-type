use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_113292085: FileFormat = FileFormat {
    id: 113_292_085,
    source_type: SourceType::Wikidata,
    name: "InterVoice file",
    extensions: &["ivc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
