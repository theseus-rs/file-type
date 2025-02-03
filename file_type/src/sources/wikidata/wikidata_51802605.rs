use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_51802605: FileFormat = FileFormat {
    id: 51_802_605,
    source_type: SourceType::Wikidata,
    name: "OS/2 Change Control File",
    extensions: &["cin"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
