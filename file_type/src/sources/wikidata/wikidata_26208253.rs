use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_26208253: FileFormat = FileFormat { id: 26_208_253, source_type: SourceType::Wikidata, name: "Office Open XML Spreadsheet Document, Strict, ISO/IEC 29500:2008, with Microsoft extensions", extensions: &["xlsx"], media_types: &["application/vnd.openxmlformats-officedocument.spreadsheetml.sheet"], internal_signatures: &[], related_formats: &[] };
