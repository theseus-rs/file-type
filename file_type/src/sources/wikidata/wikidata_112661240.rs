use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_112661240: FileFormat = FileFormat {
    id: 112_661_240,
    source_type: SourceType::Wikidata,
    name: "Autodesk Inventor Part file format",
    extensions: &["ipt"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
