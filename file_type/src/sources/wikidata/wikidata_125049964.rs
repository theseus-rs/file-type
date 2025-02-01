use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_125049964: FileFormat = FileFormat {
    id: 125_049_964,
    puid: "wikidata/125049964",
    name: "Yoshimi Vector settings file",
    extensions: &["xvy"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
