use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_34747567: FileFormat = FileFormat {
    id: 34_747_567,
    source_type: SourceType::Wikidata,
    name: "Stronghold GM1",
    extensions: &["gm1"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
