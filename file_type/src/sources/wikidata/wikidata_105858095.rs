use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858095: FileFormat = FileFormat {
    id: 105_858_095,
    source_type: SourceType::Wikidata,
    name: "Inno Setup Script (with rem)",
    extensions: &["iss"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
