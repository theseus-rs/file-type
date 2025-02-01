use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_1050471: FileFormat = FileFormat {
    id: 1_050_471,
    puid: "wikidata/1050471",
    name: "Property list",
    extensions: &["plist"],
    media_types: &["application/x-plist"],
    internal_signatures: &[],
    related_formats: &[],
};
