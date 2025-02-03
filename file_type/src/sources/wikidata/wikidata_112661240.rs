use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_112661240: FileFormat = FileFormat {
    id: 112_661_240,
    source_type: SourceType::Wikidata,
    name: "Autodesk Inventor Part file format",
    extensions: &["ipt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
