use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_51802605: FileFormat = FileFormat {
    id: 51_802_605,
    source_type: SourceType::Wikidata,
    name: "OS/2 Change Control File",
    extensions: &["cin"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
