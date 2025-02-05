use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111272301: FileFormat = FileFormat {
    id: 111_272_301,
    source_type: SourceType::Wikidata,
    name: "Ensoniq ASR instrument file",
    extensions: &["efa"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
