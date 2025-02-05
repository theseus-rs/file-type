use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_113292085: FileFormat = FileFormat {
    id: 113_292_085,
    source_type: SourceType::Wikidata,
    name: "InterVoice file",
    extensions: &["ivc"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
