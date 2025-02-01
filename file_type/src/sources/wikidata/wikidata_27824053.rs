use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27824053: FileFormat = FileFormat {
    id: 27_824_053,
    puid: "wikidata/27824053",
    name: "ar, AIX small archive format variant",
    extensions: &["a"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
