use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_61777675: FileFormat = FileFormat {
    id: 61_777_675,
    puid: "wikidata/61777675",
    name: "Microsoft Works Database for Windows, version 3a",
    extensions: &["wdb"],
    media_types: &["application/vnd.ms-works"],
    internal_signatures: &[],
    related_formats: &[],
};
