use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28600717: FileFormat = FileFormat {
    id: 28_600_717,
    puid: "wikidata/28600717",
    name: "DrawIt",
    extensions: &["drawit"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
