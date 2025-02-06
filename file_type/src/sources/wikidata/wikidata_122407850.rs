use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_122407850: FileFormat = FileFormat {
    id: 122_407_850,
    source_type: SourceType::Wikidata,
    name: "x86 Symbol File",
    extensions: &["isym"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
