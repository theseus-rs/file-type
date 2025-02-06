use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_124987792: FileFormat = FileFormat {
    id: 124_987_792,
    source_type: SourceType::Wikidata,
    name: "Dr.Geo document",
    extensions: &["fgeo"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
