use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105858095: FileFormat = FileFormat {
    id: 105_858_095,
    source_type: SourceType::Wikidata,
    name: "Inno Setup Script (with rem)",
    extensions: &["iss"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
