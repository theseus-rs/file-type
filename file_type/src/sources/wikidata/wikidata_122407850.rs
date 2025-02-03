use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_122407850: FileFormat = FileFormat {
    id: 122_407_850,
    source_type: SourceType::Wikidata,
    name: "x86 Symbol File",
    extensions: &["isym"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
