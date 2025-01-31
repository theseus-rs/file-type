use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_34287064: FileFormat = FileFormat {
    id: 34_287_064,
    puid: "wikidata/34287064",
    name: "Professor Calhoon quiz file",
    extensions: &["pc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
