use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_58007288: FileFormat = FileFormat {
    id: 58_007_288,
    source_type: SourceType::Wikidata,
    name: "VBScript file",
    extensions: &["vbs"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
