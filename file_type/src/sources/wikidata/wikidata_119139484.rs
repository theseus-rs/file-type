use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_119139484: FileFormat = FileFormat {
    id: 119_139_484,
    source_type: SourceType::Wikidata,
    name: "FreeHand Template 8",
    extensions: &["ft8"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
