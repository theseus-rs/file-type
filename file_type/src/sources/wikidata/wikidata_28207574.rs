use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28207574: FileFormat = FileFormat {
    id: 28_207_574,
    source_type: SourceType::Wikidata,
    name: "Zoomify PFF",
    extensions: &["pff"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
