use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_108837049: FileFormat = FileFormat {
    id: 108_837_049,
    source_type: SourceType::Wikidata,
    name: "Nero CD-ROM (Hybrid) Compilation",
    extensions: &["nrh"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
