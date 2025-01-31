use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_48782098: FileFormat = FileFormat {
    id: 48_782_098,
    puid: "wikidata/48782098",
    name: "Microsoft Word for MS-DOS Document, version 3",
    extensions: &["doc"],
    media_types: &["application/msword"],
    internal_signatures: &[],
    related_formats: &[],
};
