use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_50308928: FileFormat = FileFormat {
    id: 50_308_928,
    source_type: SourceType::Wikidata,
    name: "Final Draft 5-7 Document",
    extensions: &["fdr"],
    media_types: &["application/octet-stream"],
    signatures: &[],
    related_formats: &[],
};
