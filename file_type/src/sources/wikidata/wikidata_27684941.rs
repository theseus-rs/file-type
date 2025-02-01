use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27684941: FileFormat = FileFormat {
    id: 27_684_941,
    puid: "wikidata/27684941",
    name: "Microsoft Publisher file format, version 15.0",
    extensions: &["pub"],
    media_types: &["application/vnd.ms-publisher"],
    internal_signatures: &[],
    related_formats: &[],
};
