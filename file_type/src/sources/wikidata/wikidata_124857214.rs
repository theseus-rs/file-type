use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_124857214: FileFormat = FileFormat {
    id: 124_857_214,
    source_type: SourceType::Wikidata,
    name: "OCR results file",
    extensions: &["orf"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
