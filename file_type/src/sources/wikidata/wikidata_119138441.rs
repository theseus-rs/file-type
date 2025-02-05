use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_119138441: FileFormat = FileFormat {
    id: 119_138_441,
    source_type: SourceType::Wikidata,
    name: "FreeHand Template 7",
    extensions: &["ft7"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
