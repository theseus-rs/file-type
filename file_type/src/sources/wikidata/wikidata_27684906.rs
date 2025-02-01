use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27684906: FileFormat = FileFormat {
    id: 27_684_906,
    puid: "wikidata/27684906",
    name: "Microsoft Publisher file format, version 10.0",
    extensions: &["pub"],
    media_types: &["application/vnd.ms-publisher"],
    internal_signatures: &[],
    related_formats: &[],
};
