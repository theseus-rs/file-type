use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_117459518: FileFormat = FileFormat {
    id: 117_459_518,
    puid: "wikidata/117459518",
    name: "JWPUB",
    extensions: &["jwpub"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[],
    related_formats: &[],
};
