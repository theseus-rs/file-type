use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_66689214: FileFormat = FileFormat {
    id: 66_689_214,
    puid: "wikidata/66689214",
    name: "Access Blank Database Template",
    extensions: &["mdn"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
