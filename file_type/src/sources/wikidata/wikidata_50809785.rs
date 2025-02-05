use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_50809785: FileFormat = FileFormat {
    id: 50_809_785,
    source_type: SourceType::Wikidata,
    name: "Portable Database, version 3",
    extensions: &["pdb"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
