use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_45315974: FileFormat = FileFormat {
    id: 45_315_974,
    source_type: SourceType::Wikidata,
    name: "Macromedia Freehand MX file format, version 11",
    extensions: &["fh11"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
