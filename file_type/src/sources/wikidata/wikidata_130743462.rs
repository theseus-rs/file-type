use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_130743462: FileFormat = FileFormat {
    id: 130_743_462,
    source_type: SourceType::Wikidata,
    name: "Scilab source code file",
    extensions: &["sce", "sci", "tst"],
    media_types: &["text/scilab"],
    signatures: &[],
    related_formats: &[],
};
