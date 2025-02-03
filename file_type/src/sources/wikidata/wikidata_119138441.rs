use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_119138441: FileFormat = FileFormat {
    id: 119_138_441,
    source_type: SourceType::Wikidata,
    name: "FreeHand Template 7",
    extensions: &["ft7"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
