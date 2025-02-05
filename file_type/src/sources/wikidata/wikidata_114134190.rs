use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_114134190: FileFormat = FileFormat {
    id: 114_134_190,
    source_type: SourceType::Wikidata,
    name: "MOPAC dataset format",
    extensions: &["dat"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
