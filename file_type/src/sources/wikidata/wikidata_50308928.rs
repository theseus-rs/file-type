use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_50308928: FileFormat = FileFormat {
    id: 50_308_928,
    puid: "wikidata/50308928",
    name: "Final Draft 5-7 Document",
    extensions: &["fdr"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[],
    related_formats: &[],
};
