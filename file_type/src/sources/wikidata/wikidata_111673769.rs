use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111673769: FileFormat = FileFormat {
    id: 111_673_769,
    source_type: SourceType::Wikidata,
    name: "Kingsoft Spreadsheets Template",
    extensions: &["ett"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
