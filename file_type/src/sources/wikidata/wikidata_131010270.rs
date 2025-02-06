use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_131010270: FileFormat = FileFormat {
    id: 131_010_270,
    source_type: SourceType::Wikidata,
    name: "Smarty template file",
    extensions: &["tpl"],
    media_types: &["application/x-smarty"],
    signatures: &[],
    related_formats: &[],
};
