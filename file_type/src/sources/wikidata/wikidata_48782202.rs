use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_48782202: FileFormat = FileFormat {
    id: 48_782_202,
    puid: "wikidata/48782202",
    name: "Microsoft Word for MS-DOS Document, version 4",
    extensions: &["doc"],
    media_types: &["application/msword"],
    internal_signatures: &[],
    related_formats: &[],
};
