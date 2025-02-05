use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_112652668: FileFormat = FileFormat {
    id: 112_652_668,
    source_type: SourceType::Wikidata,
    name: "Gold Disk Anim format",
    extensions: &["awm"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
