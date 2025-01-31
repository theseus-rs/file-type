use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27684948: FileFormat = FileFormat {
    id: 27_684_948,
    puid: "wikidata/27684948",
    name: "Microsoft Publisher file format, version 16.0",
    extensions: &["pub"],
    media_types: &["application/vnd.ms-publisher"],
    internal_signatures: &[],
    related_formats: &[],
};
