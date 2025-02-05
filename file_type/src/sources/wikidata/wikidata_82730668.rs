use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_82730668: FileFormat = FileFormat {
    id: 82_730_668,
    source_type: SourceType::Wikidata,
    name: "Microsoft Entourage Archive",
    extensions: &["rge"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
