use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_45315877: FileFormat = FileFormat {
    id: 45_315_877,
    source_type: SourceType::Wikidata,
    name: "Macromedia Freehand file format, version 9",
    extensions: &["fh9"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
