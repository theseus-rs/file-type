use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_47498745: FileFormat = FileFormat {
    id: 47_498_745,
    source_type: SourceType::Wikidata,
    name: "Adobe Illustrator file format, version 15",
    extensions: &["ai", "pdf"],
    media_types: &["application/postscript"],
    signatures: &[],
    related_formats: &[],
};
