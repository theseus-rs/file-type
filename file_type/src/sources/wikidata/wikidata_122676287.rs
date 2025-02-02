use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_122676287: FileFormat = FileFormat {
    id: 122_676_287,
    source_type: SourceType::Wikidata,
    name: "AFX AutoFX",
    extensions: &["afx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
