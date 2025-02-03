use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_1050471: FileFormat = FileFormat {
    id: 1_050_471,
    source_type: SourceType::Wikidata,
    name: "Property list",
    extensions: &["plist"],
    media_types: &["application/x-plist"],
    internal_signatures: &[],
    related_formats: &[],
};
