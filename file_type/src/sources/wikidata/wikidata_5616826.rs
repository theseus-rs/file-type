use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_5616826: FileFormat = FileFormat {
    id: 5_616_826,
    source_type: SourceType::Wikidata,
    name: "ChordPro",
    extensions: &["cho", "chopro", "crd", "pro"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
