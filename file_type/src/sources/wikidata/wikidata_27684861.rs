use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27684861: FileFormat = FileFormat {
    id: 27_684_861,
    puid: "wikidata/27684861",
    name: "Microsoft Publisher file format, version 3.0",
    extensions: &["pub"],
    media_types: &["application/vnd.ms-publisher"],
    internal_signatures: &[],
    related_formats: &[],
};
