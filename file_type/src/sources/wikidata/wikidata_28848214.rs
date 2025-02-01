use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28848214: FileFormat = FileFormat {
    id: 28_848_214,
    puid: "wikidata/28848214",
    name: "Statistical Package for the Social Sciences data file",
    extensions: &["sav"],
    media_types: &["application/x-spss-sav"],
    internal_signatures: &[],
    related_formats: &[],
};
