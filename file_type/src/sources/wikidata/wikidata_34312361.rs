use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_34312361: FileFormat = FileFormat {
    id: 34_312_361,
    source_type: SourceType::Wikidata,
    name: "Macromedia Director, uncompressed Macintosh variant",
    extensions: &["dir", "dxr"],
    media_types: &["application/x-director"],
    signatures: &[],
    related_formats: &[],
};
