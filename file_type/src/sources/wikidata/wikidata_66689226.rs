use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_66689226: FileFormat = FileFormat {
    id: 66_689_226,
    puid: "wikidata/66689226",
    name: "Access Add-in Data",
    extensions: &["mdt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
