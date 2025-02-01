use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27684882: FileFormat = FileFormat {
    id: 27_684_882,
    puid: "wikidata/27684882",
    name: "Microsoft Publisher file format, version 8.5",
    extensions: &["pub"],
    media_types: &["application/vnd.ms-publisher"],
    internal_signatures: &[],
    related_formats: &[],
};
