use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_66662115: FileFormat = FileFormat {
    id: 66_662_115,
    source_type: SourceType::Wikidata,
    name: "Lotus Ami Pro Macro",
    extensions: &["smm"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
