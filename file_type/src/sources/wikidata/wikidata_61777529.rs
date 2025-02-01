use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_61777529: FileFormat = FileFormat {
    id: 61_777_529,
    puid: "wikidata/61777529",
    name: "Microsoft Works Database for Windows, version 3",
    extensions: &["wdb"],
    media_types: &["application/vnd.ms-works"],
    internal_signatures: &[],
    related_formats: &[],
};
