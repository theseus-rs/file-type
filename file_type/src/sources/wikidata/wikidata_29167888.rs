use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_29167888: FileFormat = FileFormat {
    id: 29_167_888,
    source_type: SourceType::Wikidata,
    name: "Personal Ancestral File, version 3",
    extensions: &["paf"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
