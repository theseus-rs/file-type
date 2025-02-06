use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_125134441: FileFormat = FileFormat {
    id: 125_134_441,
    source_type: SourceType::Wikidata,
    name: "YAM Folders",
    extensions: &["folders"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
