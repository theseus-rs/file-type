use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_29904539: FileFormat = FileFormat {
    id: 29_904_539,
    puid: "wikidata/29904539",
    name: "Statistical Analysis System output file",
    extensions: &["lst"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
