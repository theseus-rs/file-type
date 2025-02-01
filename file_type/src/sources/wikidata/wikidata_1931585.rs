use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_1931585: FileFormat = FileFormat {
    id: 1_931_585,
    puid: "wikidata/1931585",
    name: "Microsoft Digital Video Recording",
    extensions: &["dvr-ms"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
