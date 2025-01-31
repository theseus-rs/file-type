use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_61913376: FileFormat = FileFormat {
    id: 61_913_376,
    puid: "wikidata/61913376",
    name: "Microsoft Word for Macintosh Document, version 1",
    extensions: &["mcw"],
    media_types: &["application/msword"],
    internal_signatures: &[],
    related_formats: &[],
};
