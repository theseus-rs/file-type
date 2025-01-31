use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856025: FileFormat = FileFormat {
    id: 105_856_025,
    puid: "wikidata/105856025",
    name: "SharePoint Portal Server Dashboard Web Part",
    extensions: &["dwp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
