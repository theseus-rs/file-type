use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_61766587: FileFormat = FileFormat {
    id: 61_766_587,
    puid: "wikidata/61766587",
    name: "Microsoft Works Database for Windows",
    extensions: &["wdb"],
    media_types: &["application/vnd.ms-works"],
    internal_signatures: &[],
    related_formats: &[],
};
