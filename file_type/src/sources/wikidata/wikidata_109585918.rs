use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_109585918: FileFormat = FileFormat {
    id: 109_585_918,
    source_type: SourceType::Wikidata,
    name: "Painter framestack file format",
    extensions: &["frm"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
