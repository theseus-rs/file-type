use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_47498565: FileFormat = FileFormat {
    id: 47_498_565,
    source_type: SourceType::Wikidata,
    name: "Adobe Illustrator file format, version 13",
    extensions: &["ai", "pdf"],
    media_types: &["application/postscript"],
    internal_signatures: &[],
    related_formats: &[],
};
