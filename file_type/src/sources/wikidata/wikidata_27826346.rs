use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27826346: FileFormat = FileFormat {
    id: 27_826_346,
    puid: "wikidata/27826346",
    name: "Reduced resolution dataset file",
    extensions: &["rrd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
