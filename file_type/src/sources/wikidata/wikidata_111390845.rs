use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111390845: FileFormat = FileFormat {
    id: 111_390_845,
    source_type: SourceType::Wikidata,
    name: "Bryce Object File",
    extensions: &["obp"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
