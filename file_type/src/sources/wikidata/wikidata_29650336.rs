use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_29650336: FileFormat = FileFormat {
    id: 29_650_336,
    puid: "wikidata/29650336",
    name: "Personal Information Exchange",
    extensions: &["p12", "pfx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
