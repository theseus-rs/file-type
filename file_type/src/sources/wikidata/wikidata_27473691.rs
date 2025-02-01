use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27473691: FileFormat = FileFormat {
    id: 27_473_691,
    puid: "wikidata/27473691",
    name: "BIL/BIP/BSQ Header File",
    extensions: &["hdr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
