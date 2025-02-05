use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28205381: FileFormat = FileFormat {
    id: 28_205_381,
    source_type: SourceType::Wikidata,
    name: "Lytro",
    extensions: &["lfp"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
