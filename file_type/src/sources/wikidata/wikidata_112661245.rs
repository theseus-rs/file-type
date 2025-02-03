use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_112661245: FileFormat = FileFormat {
    id: 112_661_245,
    source_type: SourceType::Wikidata,
    name: "Autodesk Inventor Assembly file format",
    extensions: &["iam"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
