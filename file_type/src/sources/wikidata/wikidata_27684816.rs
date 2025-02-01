use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27684816: FileFormat = FileFormat {
    id: 27_684_816,
    puid: "wikidata/27684816",
    name: "Microsoft Publisher file format, version 1.0",
    extensions: &["pub", "pub"],
    media_types: &["application/vnd.ms-publisher", "application/x-mspublisher"],
    internal_signatures: &[],
    related_formats: &[],
};
