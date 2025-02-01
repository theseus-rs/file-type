use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_51801521: FileFormat = FileFormat {
    id: 51_801_521,
    puid: "wikidata/51801521",
    name: "Microsoft Excel Workspace",
    extensions: &["xlw"],
    media_types: &["application/vnd.ms-excel"],
    internal_signatures: &[],
    related_formats: &[],
};
