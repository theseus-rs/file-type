use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28848214: FileFormat = FileFormat {
    id: 28_848_214,
    source_type: SourceType::Wikidata,
    name: "Statistical Package for the Social Sciences data file",
    extensions: &["sav"],
    media_types: &["application/x-spss-sav"],
    internal_signatures: &[],
    related_formats: &[],
};
