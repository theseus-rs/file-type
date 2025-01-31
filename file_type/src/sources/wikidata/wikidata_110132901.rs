use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_110132901: FileFormat = FileFormat {
    id: 110_132_901,
    puid: "wikidata/110132901",
    name: "Microsoft Publisher file format, version 2010",
    extensions: &["pub"],
    media_types: &["application/x-mspublisher"],
    internal_signatures: &[],
    related_formats: &[],
};
