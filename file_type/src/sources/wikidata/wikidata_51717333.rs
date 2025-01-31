use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_51717333: FileFormat = FileFormat {
    id: 51_717_333,
    puid: "wikidata/51717333",
    name: "Microsoft Powerpoint Presentation, version 95",
    extensions: &["ppt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
