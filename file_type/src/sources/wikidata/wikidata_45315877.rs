use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_45315877: FileFormat = FileFormat {
    id: 45_315_877,
    source_type: SourceType::Wikidata,
    name: "Macromedia Freehand file format, version 9",
    extensions: &["fh9"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
