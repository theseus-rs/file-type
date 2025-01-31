use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_51333820: FileFormat = FileFormat {
    id: 51_333_820,
    puid: "wikidata/51333820",
    name: "Microsoft Powerpoint Presentation Show",
    extensions: &["pps"],
    media_types: &["application/vnd.ms-powerpoint"],
    internal_signatures: &[],
    related_formats: &[],
};
