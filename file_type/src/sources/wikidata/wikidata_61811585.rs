use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_61811585: FileFormat = FileFormat {
    id: 61_811_585,
    puid: "wikidata/61811585",
    name: "Microsoft Works Database for Windows, version 3.0b",
    extensions: &["wdb"],
    media_types: &["application/vnd.ms-works"],
    internal_signatures: &[],
    related_formats: &[],
};
