use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28919086: FileFormat = FileFormat {
    id: 28_919_086,
    puid: "wikidata/28919086",
    name: "CMX 3600 edit decision list",
    extensions: &["edl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
