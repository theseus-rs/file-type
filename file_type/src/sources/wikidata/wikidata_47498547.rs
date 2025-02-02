use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_47498547: FileFormat = FileFormat {
    id: 47_498_547,
    source_type: SourceType::Wikidata,
    name: "Adobe Illustrator file format, version 10",
    extensions: &["ai", "pdf"],
    media_types: &["application/postscript"],
    internal_signatures: &[],
    related_formats: &[],
};
