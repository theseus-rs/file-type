use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_288405: FileFormat = FileFormat {
    id: 288_405,
    source_type: SourceType::Wikidata,
    name: "hOCR",
    extensions: &["hocr", "html"],
    media_types: &["text/html", "text/vnd.hocr+html"],
    internal_signatures: &[],
    related_formats: &[],
};
