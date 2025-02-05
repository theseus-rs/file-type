use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_45315783: FileFormat = FileFormat {
    id: 45_315_783,
    source_type: SourceType::Wikidata,
    name: "Macromedia Freehand file format, version 5",
    extensions: &["fh5"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
