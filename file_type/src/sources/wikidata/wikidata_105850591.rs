use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105850591: FileFormat = FileFormat {
    id: 105_850_591,
    source_type: SourceType::Wikidata,
    name: "CryEngine Project (generic)",
    extensions: &["cryproject"],
    media_types: &["application/json"],
    internal_signatures: &[],
    related_formats: &[],
};
