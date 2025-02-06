use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_45315902: FileFormat = FileFormat {
    id: 45_315_902,
    source_type: SourceType::Wikidata,
    name: "Macromedia Freehand file format, version 8",
    extensions: &["fh8"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
