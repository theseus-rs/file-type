use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27967108: FileFormat = FileFormat {
    id: 27_967_108,
    source_type: SourceType::Wikidata,
    name: "STOS memory bank",
    extensions: &["mbk", "mbs"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
