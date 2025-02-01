use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27684919: FileFormat = FileFormat {
    id: 27_684_919,
    puid: "wikidata/27684919",
    name: "Microsoft Publisher file format, version 12.0",
    extensions: &["pub"],
    media_types: &["application/vnd.ms-publisher"],
    internal_signatures: &[],
    related_formats: &[],
};
