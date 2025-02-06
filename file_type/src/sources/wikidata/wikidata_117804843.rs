use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_117804843: FileFormat = FileFormat {
    id: 117_804_843,
    source_type: SourceType::Wikidata,
    name: "Working Model 2D File",
    extensions: &["wm"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
