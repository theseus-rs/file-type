use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850591: FileFormat = FileFormat {
    id: 105_850_591,
    source_type: SourceType::Wikidata,
    name: "CryEngine Project (generic)",
    extensions: &["cryproject"],
    media_types: &["application/json"],
    signatures: &[],
    related_formats: &[],
};
