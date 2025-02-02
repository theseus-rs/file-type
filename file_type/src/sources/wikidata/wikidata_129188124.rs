use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_129188124: FileFormat = FileFormat {
    id: 129_188_124,
    source_type: SourceType::Wikidata,
    name: "FreeFem++ source code file",
    extensions: &["edp"],
    media_types: &["text/x-freefem"],
    internal_signatures: &[],
    related_formats: &[],
};
