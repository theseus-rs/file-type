use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28207548: FileFormat = FileFormat {
    id: 28_207_548,
    puid: "wikidata/28207548",
    name: "Xerox EDMICS-RLC",
    extensions: &["rlc"],
    media_types: &["image/vnd.fujixerox.edmics-rlc"],
    internal_signatures: &[],
    related_formats: &[],
};
