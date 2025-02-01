use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_51756571: FileFormat = FileFormat {
    id: 51_756_571,
    puid: "wikidata/51756571",
    name: "AutoCAD Slide Library",
    extensions: &["slb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
