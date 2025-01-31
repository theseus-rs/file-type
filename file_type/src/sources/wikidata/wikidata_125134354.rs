use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_125134354: FileFormat = FileFormat {
    id: 125_134_354,
    puid: "wikidata/125134354",
    name: "YAM Folder Configuration",
    extensions: &["fconfig"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
