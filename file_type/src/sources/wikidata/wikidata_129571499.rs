use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_129571499: FileFormat = FileFormat {
    id: 129_571_499,
    source_type: SourceType::Wikidata,
    name: "HSAIL assembly code file",
    extensions: &["hsail"],
    media_types: &["text/x-hsail"],
    internal_signatures: &[],
    related_formats: &[],
};
