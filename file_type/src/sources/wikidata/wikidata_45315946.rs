use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_45315946: FileFormat = FileFormat {
    id: 45_315_946,
    source_type: SourceType::Wikidata,
    name: "Macromedia Freehand file format, version 7",
    extensions: &["fh7"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
