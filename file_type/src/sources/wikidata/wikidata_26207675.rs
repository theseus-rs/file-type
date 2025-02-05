use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_26207675: FileFormat = FileFormat {
    id: 26_207_675,
    source_type: SourceType::Wikidata,
    name: "Office Open XML Wordprocessing Document, Strict, ISO/IEC 29500:2008",
    extensions: &["docx"],
    media_types: &["application/vnd.openxmlformats-officedocument.wordprocessingml.document"],
    signatures: &[],
    related_formats: &[],
};
