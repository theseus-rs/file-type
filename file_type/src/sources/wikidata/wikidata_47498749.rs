use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_47498749: FileFormat = FileFormat {
    id: 47_498_749,
    source_type: SourceType::Wikidata,
    name: "Adobe Illustrator file format, version 16",
    extensions: &["ai", "pdf"],
    media_types: &["application/postscript"],
    signatures: &[],
    related_formats: &[],
};
