use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_130743462: FileFormat = FileFormat {
    id: 130_743_462,
    puid: "wikidata/130743462",
    name: "Scilab source code file",
    extensions: &["sce", "sci", "tst"],
    media_types: &["text/scilab", "text/scilab", "text/scilab"],
    internal_signatures: &[],
    related_formats: &[],
};
