use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_47498538: FileFormat = FileFormat {
    id: 47_498_538,
    source_type: SourceType::Wikidata,
    name: "Adobe Illustrator file format, version 9.0",
    extensions: &["ai", "pdf"],
    media_types: &["application/postscript"],
    signatures: &[],
    related_formats: &[],
};
