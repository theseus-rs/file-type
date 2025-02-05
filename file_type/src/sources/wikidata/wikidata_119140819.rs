use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_119140819: FileFormat = FileFormat {
    id: 119_140_819,
    source_type: SourceType::Wikidata,
    name: "FreeHand Template 9",
    extensions: &["ft9"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
