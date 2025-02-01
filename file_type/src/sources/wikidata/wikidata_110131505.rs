use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_110131505: FileFormat = FileFormat {
    id: 110_131_505,
    puid: "wikidata/110131505",
    name: "Microsoft Publisher file format, version 2003",
    extensions: &["pub"],
    media_types: &["application/x-mspublisher"],
    internal_signatures: &[],
    related_formats: &[],
};
