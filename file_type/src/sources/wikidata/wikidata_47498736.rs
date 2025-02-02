use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_47498736: FileFormat = FileFormat {
    id: 47_498_736,
    source_type: SourceType::Wikidata,
    name: "Adobe Illustrator file format, version 14",
    extensions: &["ai", "pdf"],
    media_types: &["application/postscript"],
    internal_signatures: &[],
    related_formats: &[],
};
