use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27967532: FileFormat = FileFormat {
    id: 27_967_532,
    puid: "wikidata/27967532",
    name: "DVD Information File",
    extensions: &["bup", "ifo"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
