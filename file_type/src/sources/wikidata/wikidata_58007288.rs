use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_58007288: FileFormat = FileFormat {
    id: 58_007_288,
    source_type: SourceType::Wikidata,
    name: "VBScript file",
    extensions: &["vbs"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
