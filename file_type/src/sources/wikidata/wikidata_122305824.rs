use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_122305824: FileFormat = FileFormat {
    id: 122_305_824,
    source_type: SourceType::Wikidata,
    name: "PressRelease Message",
    extensions: &["prm"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
