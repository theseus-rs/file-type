use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_61766955: FileFormat = FileFormat {
    id: 61_766_955,
    puid: "wikidata/61766955",
    name: "Microsoft Works Database for Windows, version 2.0a",
    extensions: &["wdb"],
    media_types: &["application/vnd.ms-works"],
    internal_signatures: &[],
    related_formats: &[],
};
