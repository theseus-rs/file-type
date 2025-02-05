use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111182275: FileFormat = FileFormat {
    id: 111_182_275,
    source_type: SourceType::Wikidata,
    name: "ActionScript Remote File",
    extensions: &["asr"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
