use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_113501142: FileFormat = FileFormat {
    id: 113_501_142,
    source_type: SourceType::Wikidata,
    name: "Cintel Raw Image",
    extensions: &["cri", "dvcc"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
