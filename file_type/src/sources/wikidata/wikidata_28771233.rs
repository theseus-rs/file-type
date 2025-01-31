use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28771233: FileFormat = FileFormat {
    id: 28_771_233,
    puid: "wikidata/28771233",
    name: "MINC",
    extensions: &["mnc"],
    media_types: &["application/x-minc"],
    internal_signatures: &[],
    related_formats: &[],
};
