use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_110501140: FileFormat = FileFormat {
    id: 110_501_140,
    puid: "wikidata/110501140",
    name: "Associated Signature Container Extended",
    extensions: &["asice", "sce"],
    media_types: &[
        "application/vnd.etsi.asic-e+zip",
        "application/vnd.etsi.asic-e+zip",
    ],
    internal_signatures: &[],
    related_formats: &[],
};
