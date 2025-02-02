use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_119140819: FileFormat = FileFormat {
    id: 119_140_819,
    source_type: SourceType::Wikidata,
    name: "FreeHand Template 9",
    extensions: &["ft9"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
