use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_61639409: FileFormat = FileFormat {
    id: 61_639_409,
    puid: "wikidata/61639409",
    name: "Microsoft Word for Windows Document, version 1",
    extensions: &["doc"],
    media_types: &["application/msword"],
    internal_signatures: &[],
    related_formats: &[],
};
