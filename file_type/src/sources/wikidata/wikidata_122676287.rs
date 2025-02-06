use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_122676287: FileFormat = FileFormat {
    id: 122_676_287,
    source_type: SourceType::Wikidata,
    name: "AFX AutoFX",
    extensions: &["afx"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
