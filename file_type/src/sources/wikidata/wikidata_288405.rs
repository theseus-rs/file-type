use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_288405: FileFormat = FileFormat {
    id: 288_405,
    source_type: SourceType::Wikidata,
    name: "hOCR",
    extensions: &["hocr", "html"],
    media_types: &["text/html", "text/vnd.hocr+html"],
    signatures: &[],
    related_formats: &[],
};
