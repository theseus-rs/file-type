use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_29167884: FileFormat = FileFormat {
    id: 29_167_884,
    source_type: SourceType::Wikidata,
    name: "Personal Ancestral File",
    extensions: &["paf"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
