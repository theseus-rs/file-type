use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_51801109: FileFormat = FileFormat {
    id: 51_801_109,
    puid: "wikidata/51801109",
    name: "Microsoft Excel Toolbar",
    extensions: &["xlb"],
    media_types: &["application/vnd.ms-excel"],
    internal_signatures: &[],
    related_formats: &[],
};
