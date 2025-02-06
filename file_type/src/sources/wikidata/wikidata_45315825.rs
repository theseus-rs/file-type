use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_45315825: FileFormat = FileFormat {
    id: 45_315_825,
    source_type: SourceType::Wikidata,
    name: "Macromedia Freehand file format, version 10",
    extensions: &["fh10"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
