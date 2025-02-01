use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_48782238: FileFormat = FileFormat {
    id: 48_782_238,
    puid: "wikidata/48782238",
    name: "Microsoft Word for MS-DOS Document, version 5",
    extensions: &["doc"],
    media_types: &["application/msword"],
    internal_signatures: &[],
    related_formats: &[],
};
