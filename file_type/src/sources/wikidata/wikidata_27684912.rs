use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27684912: FileFormat = FileFormat {
    id: 27_684_912,
    puid: "wikidata/27684912",
    name: "Microsoft Publisher file format, version 11.0",
    extensions: &["pub"],
    media_types: &["application/vnd.ms-publisher"],
    internal_signatures: &[],
    related_formats: &[],
};
