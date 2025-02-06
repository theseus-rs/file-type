use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_34740349: FileFormat = FileFormat {
    id: 34_740_349,
    source_type: SourceType::Wikidata,
    name: "Softdisk Family Tree 1 Comment Data",
    extensions: &["fcm"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
