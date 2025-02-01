use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_125048463: FileFormat = FileFormat {
    id: 125_048_463,
    puid: "wikidata/125048463",
    name: "Yoshimi Scale Settings file",
    extensions: &["xsz"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
