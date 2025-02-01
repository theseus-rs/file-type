use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_113470100: FileFormat = FileFormat {
    id: 113_470_100,
    puid: "wikidata/113470100",
    name: "Microsoft Word for MS-DOS Document, version 6.0",
    extensions: &["doc"],
    media_types: &["application/msword"],
    internal_signatures: &[],
    related_formats: &[],
};
