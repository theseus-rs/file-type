use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_130743462: FileFormat = FileFormat {
    id: 130_743_462,
    source_type: SourceType::Wikidata,
    name: "Scilab source code file",
    extensions: &["sce", "sci", "tst"],
    media_types: &["text/scilab"],
    internal_signatures: &[],
    related_formats: &[],
};
