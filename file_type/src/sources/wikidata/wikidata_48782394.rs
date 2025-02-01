use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_48782394: FileFormat = FileFormat {
    id: 48_782_394,
    puid: "wikidata/48782394",
    name: "Microsoft Word for MS-DOS Document, version 5.5",
    extensions: &["doc"],
    media_types: &["application/msword"],
    internal_signatures: &[],
    related_formats: &[],
};
