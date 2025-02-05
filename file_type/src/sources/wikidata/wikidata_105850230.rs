use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850230: FileFormat = FileFormat {
    id: 105_850_230,
    source_type: SourceType::Wikidata,
    name: "EISA add-on card Configuration (with rem)",
    extensions: &["cfg"],
    media_types: &["application/octet-stream"],
    signatures: &[],
    related_formats: &[],
};
