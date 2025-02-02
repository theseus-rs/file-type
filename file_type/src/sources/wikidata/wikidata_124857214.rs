use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_124857214: FileFormat = FileFormat {
    id: 124_857_214,
    source_type: SourceType::Wikidata,
    name: "OCR results file",
    extensions: &["orf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
