use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28207574: FileFormat = FileFormat {
    id: 28_207_574,
    source_type: SourceType::Wikidata,
    name: "Zoomify PFF",
    extensions: &["pff"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
