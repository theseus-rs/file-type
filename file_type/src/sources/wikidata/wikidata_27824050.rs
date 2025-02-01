use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27824050: FileFormat = FileFormat {
    id: 27_824_050,
    puid: "wikidata/27824050",
    name: "ar, AIX big archive format variant",
    extensions: &["a"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
