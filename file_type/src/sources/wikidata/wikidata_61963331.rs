use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_61963331: FileFormat = FileFormat {
    id: 61_963_331,
    puid: "wikidata/61963331",
    name: "pulse EKKO data file",
    extensions: &["dt1"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
