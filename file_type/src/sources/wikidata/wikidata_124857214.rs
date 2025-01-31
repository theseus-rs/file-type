use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_124857214: FileFormat = FileFormat {
    id: 124_857_214,
    puid: "wikidata/124857214",
    name: "OCR results file",
    extensions: &["orf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
