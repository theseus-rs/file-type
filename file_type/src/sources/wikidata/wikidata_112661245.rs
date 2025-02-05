use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_112661245: FileFormat = FileFormat {
    id: 112_661_245,
    source_type: SourceType::Wikidata,
    name: "Autodesk Inventor Assembly file format",
    extensions: &["iam"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
