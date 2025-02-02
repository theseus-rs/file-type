use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_119139484: FileFormat = FileFormat {
    id: 119_139_484,
    source_type: SourceType::Wikidata,
    name: "FreeHand Template 8",
    extensions: &["ft8"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
