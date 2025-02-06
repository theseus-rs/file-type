use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_1050471: FileFormat = FileFormat {
    id: 1_050_471,
    source_type: SourceType::Wikidata,
    name: "Property list",
    extensions: &["plist"],
    media_types: &["application/x-plist"],
    signatures: &[],
    related_formats: &[],
};
