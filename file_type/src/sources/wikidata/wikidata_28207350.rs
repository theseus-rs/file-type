use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28207350: FileFormat = FileFormat {
    id: 28_207_350,
    puid: "wikidata/28207350",
    name: "Video Display Adapter",
    extensions: &["vda"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
