use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111263007: FileFormat = FileFormat {
    id: 111_263_007,
    source_type: SourceType::Wikidata,
    name: "Velvet Studio sample",
    extensions: &["ase"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
