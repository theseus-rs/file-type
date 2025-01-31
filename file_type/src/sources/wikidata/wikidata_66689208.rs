use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_66689208: FileFormat = FileFormat {
    id: 66_689_208,
    puid: "wikidata/66689208",
    name: "Access Database (Pocket Access for Windows CE)",
    extensions: &["cdb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
