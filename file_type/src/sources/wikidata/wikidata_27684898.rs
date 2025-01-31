use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27684898: FileFormat = FileFormat {
    id: 27_684_898,
    puid: "wikidata/27684898",
    name: "Microsoft Publisher file format, version 9.0",
    extensions: &["pub"],
    media_types: &["application/vnd.ms-publisher"],
    internal_signatures: &[],
    related_formats: &[],
};
