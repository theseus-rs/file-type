use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27684873: FileFormat = FileFormat {
    id: 27_684_873,
    puid: "wikidata/27684873",
    name: "Microsoft Publisher file format, version 8.0",
    extensions: &["pub"],
    media_types: &["application/vnd.ms-publisher"],
    internal_signatures: &[],
    related_formats: &[],
};
