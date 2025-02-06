use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856025: FileFormat = FileFormat {
    id: 105_856_025,
    source_type: SourceType::Wikidata,
    name: "SharePoint Portal Server Dashboard Web Part",
    extensions: &["dwp"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
