use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27979381: FileFormat = FileFormat {
    id: 27_979_381,
    source_type: SourceType::Wikidata,
    name: "Blu-ray Clip info",
    extensions: &["clp", "clpi", "cpi"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
