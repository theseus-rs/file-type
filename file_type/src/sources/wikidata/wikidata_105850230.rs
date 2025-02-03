use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105850230: FileFormat = FileFormat {
    id: 105_850_230,
    source_type: SourceType::Wikidata,
    name: "EISA add-on card Configuration (with rem)",
    extensions: &["cfg"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[],
    related_formats: &[],
};
